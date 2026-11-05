mod consts;
pub mod sha512;

// Ed25519 signature verification (RFC 8032, section 5.1.7), cofactored, over noir-lang's bignum.
// Our code; the structure follows the RFC. Decoding is strict: canonical y and x, x parity equal to
// the sign bit, S < L, and a public key of small order is refused.
use bignum::{BigNum, ED25519_Fq as Fq, ED25519_Fr as Fr};
use crate::consts::{ED_BX, ED_BY, ED_D};
use crate::sha512::sha512;

pub struct Point {
    x: Fq,
    y: Fq,
}

fn identity() -> Point {
    Point { x: Fq::zero(), y: Fq::one() }
}

// Complete twisted Edwards addition with a = -1. Ed25519's d is not a square, so the
// denominators are never zero for points on the curve, and doubling uses the same formula.
fn add(p: Point, q: Point) -> Point {
    let x1x2 = p.x * q.x;
    let y1y2 = p.y * q.y;
    let x1y2 = p.x * q.y;
    let y1x2 = p.y * q.x;
    let t = Fq::from_limbs(ED_D) * x1x2 * y1y2;
    let one = Fq::one();
    Point { x: (x1y2 + y1x2) / (one + t), y: (y1y2 + x1x2) / (one - t) }
}

fn select(bit: bool, a: Point, b: Point) -> Point {
    if bit {
        a
    } else {
        b
    }
}

fn assert_on_curve(p: Point) {
    let x2 = p.x * p.x;
    let y2 = p.y * p.y;
    // -x^2 + y^2 = 1 + d x^2 y^2
    assert(y2 - x2 == Fq::one() + Fq::from_limbs(ED_D) * x2 * y2, "point not on curve");
}

fn is_identity(p: Point) -> bool {
    (p.x == Fq::zero()) & (p.y == Fq::one())
}

fn times8(p: Point) -> Point {
    let p2 = add(p, p);
    let p4 = add(p2, p2);
    add(p4, p4)
}

// Strictly below the modulus: validate_in_field allows the modulus itself, so that is refused too.
fn assert_canonical_fq(v: Fq) {
    v.validate_in_range();
    v.validate_in_field();
    assert(v.get_limbs() != Fq::modulus().get_limbs(), "not canonical");
}

fn assert_canonical_fr(v: Fr) {
    v.validate_in_range();
    v.validate_in_field();
    assert(v.get_limbs() != Fr::modulus().get_limbs(), "S or k not below L");
}

// RFC 8032 5.1.3. The prover supplies x; the circuit checks it instead of taking a square root.
fn decode(enc: [u8; 32], x_limbs: [u128; 3]) -> Point {
    let sign = enc[31] >> 7;
    let mut y_bytes = enc;
    y_bytes[31] = enc[31] & 0x7f;
    let y = Fq::from_le_bytes(y_bytes);
    assert_canonical_fq(y);
    let x = Fq::from_limbs(x_limbs);
    assert_canonical_fq(x);
    let x_bytes = x.to_le_bytes();
    assert((x_bytes[0] & 1) == sign, "x parity differs from the sign bit");
    let p = Point { x, y };
    assert_on_curve(p);
    p
}

fn bits_le(bytes: [u8; 32]) -> [bool; 256] {
    let mut out: [bool; 256] = [false; 256];
    for i in 0..32 {
        let bs: [bool; 8] = (bytes[i] as Field).to_le_bits();
        for j in 0..8 {
            out[i * 8 + j] = bs[j];
        }
    }
    out
}

// Interpret a 64-byte digest as a little-endian integer and reduce it modulo L.
fn reduce_wide(h: [u8; 64]) -> Fr {
    let base = Fr::from(0x100000000);
    let mut acc = Fr::zero();
    for i in 0..16 {
        let j = 15 - i;
        let word = (h[4 * j] as Field)
            + (h[4 * j + 1] as Field) * 0x100
            + (h[4 * j + 2] as Field) * 0x10000
            + (h[4 * j + 3] as Field) * 0x1000000;
        acc = acc * base + Fr::from(word);
    }
    acc
}

// Asserts that sig is a valid signature by pk over msg. P is the SHA-512 padded length of 64 + N bytes.
pub fn verify<let N: u32, let P: u32>(
    pk: [u8; 32],
    sig: [u8; 64],
    msg: [u8; N],
    pk_x: [u128; 3],
    r_x: [u128; 3],
) {
    let a = decode(pk, pk_x);
    assert(!is_identity(times8(a)), "public key of small order");

    let mut r_enc: [u8; 32] = [0; 32];
    let mut s_enc: [u8; 32] = [0; 32];
    for i in 0..32 {
        r_enc[i] = sig[i];
        s_enc[i] = sig[32 + i];
    }
    let r = decode(r_enc, r_x);
    let s = Fr::from_le_bytes(s_enc);
    assert_canonical_fr(s);

    // k = SHA-512(R || A || M) mod L, over the encodings as given.
    let mut pre: [u8; 64 + N] = [0; 64 + N];
    for i in 0..32 {
        pre[i] = r_enc[i];
        pre[32 + i] = pk[i];
    }
    for i in 0..N {
        pre[64 + i] = msg[i];
    }
    let k = reduce_wide(sha512::<64 + N, P>(pre));
    assert_canonical_fr(k);

    // [S]B + [k](-A), one shared doubling chain, scalars below L < 2^253.
    let b = Point { x: Fq::from_limbs(ED_BX), y: Fq::from_limbs(ED_BY) };
    let neg_a = Point { x: Fq::zero() - a.x, y: a.y };
    let b_neg_a = add(b, neg_a);
    let sb = bits_le(s.to_le_bytes());
    let kb = bits_le(k.to_le_bytes());
    let mut acc = identity();
    for step in 0..253 {
        let i = 252 - step;
        acc = add(acc, acc);
        let t = select(sb[i], select(kb[i], b_neg_a, b), select(kb[i], neg_a, identity()));
        acc = add(acc, t);
    }
    // Cofactored: [8]([S]B - [k]A) = [8]R.
    let lhs = times8(acc);
    let rhs = times8(r);
    assert(lhs.x == rhs.x, "signature does not verify");
    assert(lhs.y == rhs.y, "signature does not verify");
}
