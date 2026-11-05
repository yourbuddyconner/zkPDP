//! Turn a corpus set into inputs for the hand-written TinyTodo circuit.
//!
//!   circuit-tool <repo> <set> <circuit dir>
//!
//! Writes `src/consts.nr` (the policy and schema hashes the circuit is for)
//! and `inputs/<n>.toml` plus `inputs/index.json`, one per corpus request.
//! With a fourth argument `signed`, the inputs are for the signed-store circuit.

use envelope::{canonical_request, parse_hex_hash, sha256, uid_text, RecordStore, MAX_ANC, MAX_ATTRS};
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const MAX_DEPTH: usize = 12;

fn arr(b: &[u8]) -> String {
    format!("[{}]", b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
}

fn bounded(name: &str, s: &str, max: usize) -> String {
    assert!(s.len() <= max, "{name} is longer than {max}");
    let mut b = s.as_bytes().to_vec();
    b.resize(max, 0);
    format!("[{name}]\nstorage = {}\nlen = {}\n\n", arr(&b), s.len())
}

/// The authority's test key: fixed, public, and for this corpus only.
fn test_authority() -> p256::ecdsa::SigningKey {
    let seed = sha256(b"zk-pdp test authority key, not a secret");
    p256::ecdsa::SigningKey::from_slice(&seed).expect("scalar in range")
}

const TEST_EPOCH: u64 = 1;

/// The signed bytes: tag, root, epoch big-endian.
fn signed_bytes(root: &[u8; 32], epoch: u64) -> Vec<u8> {
    let mut m = b"zk-pdp-root-v0".to_vec();
    m.extend_from_slice(root);
    m.extend_from_slice(&epoch.to_be_bytes());
    m
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let signed = a.get(4).map(String::as_str) == Some("signed");
    let (repo, set, out) = (Path::new(&a[1]), &a[2], Path::new(&a[3]));
    let dir = repo.join("corpus/sets").join(set);
    let manifest: Value = serde_json::from_slice(&fs::read(repo.join("corpus/MANIFEST.json")).unwrap()).unwrap();
    let m = &manifest["sets"][set.as_str()];
    let policy_hash = parse_hex_hash(m["policy_hash"].as_str().unwrap()).unwrap();
    let schema_hash = parse_hex_hash(m["schema_hash"].as_str().unwrap()).unwrap();
    let entities: Value = serde_json::from_slice(&fs::read(dir.join("entities.json")).unwrap()).unwrap();
    let store = RecordStore::new(&entities, &envelope::Layouts::new()).expect("record store");

    fs::write(
        out.join("src/consts.nr"),
        format!(
            "// Written by circuit-tool from corpus set `{set}`. Do not edit.\n\
             pub global POLICY_HASH: [u8; 32] = {};\n\
             pub global SCHEMA_HASH: [u8; 32] = {};\n",
            arr(&policy_hash),
            arr(&schema_hash)
        ),
    )
    .unwrap();

    fs::create_dir_all(out.join("inputs")).unwrap();
    let mut index = Vec::new();
    for (n, line) in fs::read_to_string(dir.join("requests.jsonl")).unwrap().lines().enumerate() {
        let r: Value = serde_json::from_str(line).unwrap();
        let req = &r["request"];
        let f = |p: &str| req.pointer(p).and_then(Value::as_str).unwrap().to_owned();
        let mut t = String::new();
        writeln!(t, "request_hash = {}", arr(&sha256(&canonical_request(req)))).unwrap();
        writeln!(t, "policy_hash = {}", arr(&policy_hash)).unwrap();
        writeln!(t, "schema_hash = {}", arr(&schema_hash)).unwrap();
        if signed {
            use p256::ecdsa::{signature::hazmat::PrehashSigner, Signature};
            let key = test_authority();
            let point = key.verifying_key().to_encoded_point(false);
            writeln!(t, "authority_x = {}", arr(point.x().unwrap())).unwrap();
            writeln!(t, "authority_y = {}", arr(point.y().unwrap())).unwrap();
            writeln!(t, "epoch = {TEST_EPOCH}").unwrap();
            let sig: Signature = key.sign_prehash(&sha256(&signed_bytes(&store.root, TEST_EPOCH))).expect("sign");
            let sig = sig.normalize_s().unwrap_or(sig);
            writeln!(t, "signature = {}", arr(&sig.to_bytes())).unwrap();
        }
        writeln!(t, "entities_root = {}", arr(&store.root)).unwrap();
        writeln!(t, "decision = {}\n", r["oracle"]["decision"]).unwrap();
        t += &bounded("nonce", &f("/context/nonce"), 32);
        t += &bounded("subject_type", &f("/subject/type"), 16);
        t += &bounded("subject_id", &f("/subject/id"), 32);
        t += &bounded("action", &f("/action/name"), 16);
        t += &bounded("resource_type", &f("/resource/type"), 16);
        t += &bounded("resource_id", &f("/resource/id"), 32);
        for (name, who) in [("principal", "subject"), ("resource", "resource")] {
            let uid = uid_text(&req[who]).unwrap();
            let (rec, path) = store.open(&uid).unwrap_or_else(|| panic!("{uid} is not in the store"));
            assert!(path.len() <= MAX_DEPTH);
            let mut sib = vec![[0u8; 32]; MAX_DEPTH];
            let mut left = vec![false; MAX_DEPTH];
            for (i, s) in path.iter().enumerate() {
                sib[i] = s.sibling;
                left[i] = s.sibling_is_left;
            }
            let rows = |v: &[[u8; 32]]| format!("[{}]", v.iter().map(|x| arr(x)).collect::<Vec<_>>().join(", "));
            writeln!(t, "[{name}]").unwrap();
            writeln!(t, "present = {}", rec.present).unwrap();
            writeln!(t, "attrs = {}", rows(&rec.attrs[..MAX_ATTRS])).unwrap();
            writeln!(t, "ancestors = {}", rows(&rec.ancestors[..MAX_ANC])).unwrap();
            writeln!(t, "path = {}", rows(&sib)).unwrap();
            writeln!(t, "sibling_is_left = [{}]", left.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")).unwrap();
            writeln!(t, "path_len = {}\n", path.len()).unwrap();
        }
        fs::write(out.join(format!("inputs/{n}.toml")), t).unwrap();
        index.push(json!({ "n": n, "name": r["name"], "decision": r["oracle"]["decision"], "request_hash": r["request_hash"] }));
    }
    let idx = json!({
        "set": set,
        "policy_hash": m["policy_hash"],
        "schema_hash": m["schema_hash"],
        "entities": { "type": "merkle-sha256-record-v0", "root": envelope::hex_hash(&store.root) },
        "requests": index,
    });
    fs::write(out.join("inputs/index.json"), serde_json::to_string_pretty(&idx).unwrap() + "\n").unwrap();
    println!("{} inputs, root {}", idx["requests"].as_array().unwrap().len(), envelope::hex_hash(&store.root));
}
