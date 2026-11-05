mod rfc8032;
// An authority's Ed25519 signature over a store root, as in the signed mode, which uses P-256.
// Message: "zk-pdp-root-v0" || root || epoch, big-endian, 54 bytes.

global TAG: [u8; 14] = [122, 107, 45, 112, 100, 112, 45, 114, 111, 111, 116, 45, 118, 48];

fn main(pk: pub [u8; 32], root: pub [u8; 32], epoch: pub u64, sig: [u8; 64], pk_x: [u128; 3], r_x: [u128; 3]) {
    let mut msg: [u8; 54] = [0; 54];
    for i in 0..14 {
        msg[i] = TAG[i];
    }
    for i in 0..32 {
        msg[14 + i] = root[i];
    }
    for i in 0..8 {
        msg[46 + i] = ((epoch >> (56 - 8 * i as u64)) & 0xff) as u8;
    }
    ed25519::verify::<54, 256>(pk, sig, msg, pk_x, r_x);
}

#[test]
fn sha512_matches_reference() {
    // SHA-512("abc"), FIPS 180-4 example.
    let d = ed25519::sha512::sha512::<3, 128>([0x61, 0x62, 0x63]);
    assert(d[0] == 0xdd);
    assert(d[1] == 0xaf);
    assert(d[63] == 0x9f);
}
