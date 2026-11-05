// SHA-512 (FIPS 180-4) over a message of fixed length N, padded to P bytes.
// Fixed length on purpose: no variable-length padding logic to get wrong.
use crate::consts::{SHA512_H0, SHA512_K};

fn wadd<let M: u32>(xs: [u64; M]) -> u64 {
    let mut s: Field = 0;
    for i in 0..M {
        s += xs[i] as Field;
    }
    // Truncating cast: the sum of at most 5 words is below 2^67, and the cast keeps its low 64 bits.
    s as u64
}

fn rotr(x: u64, n: u64) -> u64 {
    // The low n bits move to the top. Both halves fit in 64 bits, so no shift overflows.
    (x >> n) | ((x & ((1 << n) - 1)) << (64 - n))
}

fn compress(state: [u64; 8], block: [u8; 128]) -> [u64; 8] {
    let mut w: [u64; 80] = [0; 80];
    for i in 0..16 {
        let mut v: u64 = 0;
        for j in 0..8 {
            v = (v << 8) | (block[i * 8 + j] as u64);
        }
        w[i] = v;
    }
    for i in 16..80 {
        let s0 = rotr(w[i - 15], 1) ^ rotr(w[i - 15], 8) ^ (w[i - 15] >> 7);
        let s1 = rotr(w[i - 2], 19) ^ rotr(w[i - 2], 61) ^ (w[i - 2] >> 6);
        w[i] = wadd([w[i - 16], s0, w[i - 7], s1]);
    }
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];
    for i in 0..80 {
        let s1 = rotr(e, 14) ^ rotr(e, 18) ^ rotr(e, 41);
        let ch = (e & f) ^ (!e & g);
        let t1 = wadd([h, s1, ch, SHA512_K[i], w[i]]);
        let s0 = rotr(a, 28) ^ rotr(a, 34) ^ rotr(a, 39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = wadd([s0, maj]);
        h = g;
        g = f;
        f = e;
        e = wadd([d, t1]);
        d = c;
        c = b;
        b = a;
        a = wadd([t1, t2]);
    }
    [
        wadd([state[0], a]), wadd([state[1], b]), wadd([state[2], c]), wadd([state[3], d]),
        wadd([state[4], e]), wadd([state[5], f]), wadd([state[6], g]), wadd([state[7], h]),
    ]
}

pub fn sha512<let N: u32, let P: u32>(msg: [u8; N]) -> [u8; 64] {
    assert(P == ((N + 17 + 127) / 128) * 128, "P must be the padded length");
    let mut padded: [u8; P] = [0; P];
    for i in 0..N {
        padded[i] = msg[i];
    }
    padded[N] = 0x80;
    let bits: u64 = (N as u64) * 8;
    for i in 0..8 {
        padded[P - 1 - i] = ((bits >> (8 * i as u64)) & 0xff) as u8;
    }
    let mut state = SHA512_H0;
    for blk in 0..(P / 128) {
        let mut block: [u8; 128] = [0; 128];
        for i in 0..128 {
            block[i] = padded[blk * 128 + i];
        }
        state = compress(state, block);
    }
    let mut out: [u8; 64] = [0; 64];
    for i in 0..8 {
        for j in 0..8 {
            out[i * 8 + j] = ((state[i] >> (56 - 8 * j as u64)) & 0xff) as u8;
        }
    }
    out
}
