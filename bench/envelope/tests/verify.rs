use envelope::*;
use serde_json::{json, Value};

struct Mock(bool);
impl SealVerifier for Mock {
    fn verify(&self, _f: &str, _e: &[u8; 32], _j: &Journal, seal: &[u8]) -> Result<(), String> {
        if self.0 && seal == b"good" { Ok(()) } else { Err("bad seal".into()) }
    }
}

fn request() -> Value {
    json!({
        "subject": {"type": "User", "id": "alice"},
        "action": {"name": "GetList"},
        "resource": {"type": "List", "id": "l0"},
        "context": {"nonce": "n-1"}
    })
}

fn setup() -> (Value, Value, Pins) {
    let req = request();
    let j = Journal {
        v: 0,
        request_hash: request_hash(&req),
        policy_hash: [1; 32],
        schema_hash: [2; 32],
        entities: EntityCommitment { kind: ENTITIES_SHA256_JCS_V0.into(), root: [3; 32] },
        nonce: "n-1".into(),
        decision: true,
        reasons: None,
    };
    let resp = response(&j, FORMAT_RISC0_SUCCINCT, &[9; 32], b"good");
    let pins = Pins {
        evaluators: vec![(FORMAT_RISC0_SUCCINCT.into(), [9; 32])],
        policy_hashes: vec![[1; 32]],
        schema_hash: Some([2; 32]),
        entity_roots: vec![[3; 32]],
        nonce: Some("n-1".into()),
    };
    (req, resp, pins)
}

#[test]
fn accepts_a_good_response() {
    let (req, resp, pins) = setup();
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Ok(true));
}

#[test]
fn step1_refuses_unknown_format() {
    let (req, mut resp, pins) = setup();
    resp["context"]["proof"]["format"] = json!("other");
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Format));
}

#[test]
fn step2_refuses_wrong_evaluator() {
    let (req, resp, mut pins) = setup();
    pins.evaluators[0].1 = [8; 32];
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Evaluator));
}

#[test]
fn step3_refuses_bad_seal() {
    let (req, resp, pins) = setup();
    assert!(matches!(verify(&req, &resp, &pins, &Mock(false)), Err(Refusal::Seal(_))));
}

#[test]
fn step4_refuses_a_different_request() {
    let (mut req, resp, pins) = setup();
    req["resource"]["id"] = json!("l1");
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Request));
}

#[test]
fn step4_ignores_key_order_and_extra_members() {
    let (_, resp, pins) = setup();
    let req: Value = serde_json::from_str(
        r#"{"extra":1,"context":{"nonce":"n-1"},"resource":{"id":"l0","type":"List"},
            "action":{"name":"GetList"},"subject":{"id":"alice","type":"User"}}"#,
    )
    .unwrap();
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Ok(true));
}

#[test]
fn step5_refuses_unlisted_policy() {
    let (req, resp, mut pins) = setup();
    pins.policy_hashes = vec![[7; 32]];
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Policy));
}

#[test]
fn step6_refuses_wrong_schema() {
    let (req, resp, mut pins) = setup();
    pins.schema_hash = Some([7; 32]);
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Schema));
}

#[test]
fn step7_refuses_stale_entities() {
    let (req, resp, mut pins) = setup();
    pins.entity_roots = vec![[7; 32]];
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Entities));
}

#[test]
fn step8_refuses_replay_under_a_new_nonce() {
    // The verifier issued n-2 and hashes its own request. A stored response for n-1 fails at the request.
    let (mut req, resp, mut pins) = setup();
    req["context"]["nonce"] = json!("n-2");
    pins.nonce = Some("n-2".into());
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Request));
    // And a verifier that forgot to put the nonce in its request still catches it.
    let (req, resp, mut pins) = setup();
    pins.nonce = Some("n-2".into());
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Nonce));
}

#[test]
fn step9_refuses_a_flipped_decision() {
    let (req, mut resp, pins) = setup();
    resp["decision"] = json!(false);
    assert_eq!(verify(&req, &resp, &pins, &Mock(true)), Err(Refusal::Decision));
}

#[test]
fn entities_hash_ignores_order() {
    let a = json!([{"uid":{"type":"User","id":"b"},"attrs":{},"parents":[]},
                   {"uid":{"type":"User","id":"a"},"attrs":{},"parents":[]}]);
    let b = json!([{"parents":[],"attrs":{},"uid":{"id":"a","type":"User"}},
                   {"uid":{"type":"User","id":"b"},"attrs":{},"parents":[]}]);
    assert_eq!(canonical_entities(&a).unwrap(), canonical_entities(&b).unwrap());
    assert_eq!(entities_merkle_root(&a).unwrap(), entities_merkle_root(&b).unwrap());
}

#[test]
fn merkle_paths_verify_for_every_leaf_and_size() {
    for n in 1..20usize {
        let leaves: Vec<[u8; 32]> = (0..n).map(|i| sha256(&[i as u8])).collect();
        let root = merkle_root(&leaves);
        for i in 0..n {
            let p = merkle_path(&leaves, i);
            assert!(merkle_verify(&leaves[i], &p, &root), "n={n} i={i}");
            assert!(!merkle_verify(&sha256(b"x"), &p, &root));
        }
    }
}

#[test]
fn record_store_opens_every_entity_and_closes_ancestors() {
    let es = json!([
        {"uid":{"type":"Team","id":"a"},"attrs":{},"parents":[]},
        {"uid":{"type":"Team","id":"b"},"attrs":{},"parents":[{"type":"Team","id":"a"}]},
        {"uid":{"type":"User","id":"u"},"attrs":{"level":7,"home":"x"},"parents":[{"type":"Team","id":"b"}]},
        {"uid":{"type":"List","id":"l"},"attrs":{"owner":{"__entity":{"type":"User","id":"u"}}},"parents":[]}
    ]);
    let store = RecordStore::new(&es, &Layouts::new()).unwrap();
    for r in &store.records {
        let (rec, path) = store.open(&r.uid_text).unwrap();
        assert!(merkle_verify(&rec.leaf(), &path, &store.root));
    }
    let (u, _) = store.open("User::\"u\"").unwrap();
    // u is in b directly and in a through b.
    assert!(u.ancestors.contains(&uid_hash("Team::\"a\"")));
    assert!(u.ancestors.contains(&uid_hash("Team::\"b\"")));
    // Without a layout, attributes are the present ones in name order: home, level.
    assert_eq!(u.present, 0b11);
    assert_eq!(u.attrs[0], sha256(b"x"));
    assert_eq!(u.attrs[1][31], 7);
    let (l, _) = store.open("List::\"l\"").unwrap();
    assert_eq!(l.attrs[0], uid_hash("User::\"u\""));
}

#[test]
fn record_layout_fixes_positions_and_opens_sets_and_records() {
    let layout = vec![
        ("editors".to_string(), Kind::Set(Box::new(Kind::Leaf))),
        ("nick".to_string(), Kind::Leaf),
        ("perms".to_string(), Kind::Record(vec![("level".to_string(), Kind::Leaf)])),
    ];
    let mut layouts = Layouts::new();
    layouts.insert("Doc".into(), layout);
    // `nick` is optional and absent: its slot stays zero and later slots do not move.
    let es = json!([{"uid":{"type":"Doc","id":"d"},"parents":[],"attrs":{
        "editors":[{"__entity":{"type":"User","id":"b"}},{"__entity":{"type":"User","id":"a"}}],
        "perms":{"level":3}}}]);
    let store = RecordStore::new(&es, &layouts).unwrap();
    let (d, _) = store.open("Doc::\"d\"").unwrap();
    assert_eq!(d.present, 0b101);
    assert_eq!(d.attrs[1], [0u8; 32]);
    let set = set_block(&es[0]["attrs"]["editors"], &Kind::Leaf).unwrap();
    assert_eq!(d.attrs[0], sha256(&set.concat()));
    assert!(set.contains(&uid_hash("User::\"a\"")) && set.contains(&uid_hash("User::\"b\"")));
    assert!(set[0] < set[1]);
    let (p, slots) = record_block(es[0]["attrs"]["perms"].as_object().unwrap(), &[("level".to_string(), Kind::Leaf)]).unwrap();
    assert_eq!(d.attrs[2], sha256(&block_bytes(p, &slots)));
    // An attribute outside the layout is refused.
    let bad = json!([{"uid":{"type":"Doc","id":"d"},"parents":[],"attrs":{"other":1}}]);
    assert!(RecordStore::new(&bad, &layouts).is_err());
}
