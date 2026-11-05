//! The fair-comparison guest (E1b): Cedar over the same two records the
//! circuits open, instead of the whole store.
//!
//! The guest takes the principal's and the resource's JSON, recomputes each
//! `merkle-sha256-record-v0` leaf from it with the SHA-256 precompile, checks
//! the path to the root, and hands Cedar only those two entities with their
//! closed ancestors as parents. The policy text is parsed here. The schema is
//! hashed and not parsed: the authority validated the store when it committed
//! to it. Unlike the circuits, this guest does not refuse a request outside the
//! schema. Cedar denies it.

use std::io::{Read, Write};
use std::str::FromStr;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet, Request,
};
use envelope::{EntityCommitment, Journal};
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use serde_json::Value;

const MAX_ATTRS: usize = 8;
const MAX_ANC: usize = 16;
const KIND: &str = "merkle-sha256-record-v0";

fn sha(bytes: &[u8]) -> [u8; 32] {
    Impl::hash_bytes(bytes).as_bytes().try_into().unwrap()
}

fn take<'a>(buf: &'a [u8], at: &mut usize) -> &'a [u8] {
    let n = u32::from_le_bytes(buf[*at..*at + 4].try_into().unwrap()) as usize;
    let out = &buf[*at + 4..*at + 4 + n];
    *at += 4 + n;
    out
}

fn uid(type_name: &str, id: &str) -> EntityUid {
    EntityUid::from_type_name_and_id(EntityTypeName::from_str(type_name).expect("type"), EntityId::new(id))
}

fn uid_text(v: &Value) -> Option<String> {
    let u = v.get("__entity").unwrap_or(v);
    let (t, id) = (u.get("type")?.as_str()?, u.get("id")?.as_str()?);
    Some(format!("{t}::\"{}\"", id.replace('\\', "\\\\").replace('"', "\\\"")))
}

fn is_ref(v: &Value) -> bool {
    let u = v.get("__entity").unwrap_or(v);
    matches!(u.as_object(), Some(o) if o.len() == 2 && o.get("type").map_or(false, Value::is_string) && o.get("id").map_or(false, Value::is_string))
}

// The encoding of `bench/envelope/src/records.rs`, without a layout.
fn slot(v: &Value) -> [u8; 32] {
    if is_ref(v) {
        return sha(uid_text(v).unwrap().as_bytes());
    }
    match v {
        Value::Number(n) if n.is_i64() => {
            let mut s = [0u8; 32];
            s[24..].copy_from_slice(&n.as_i64().unwrap().to_be_bytes());
            s
        }
        Value::Bool(b) => {
            let mut s = [0u8; 32];
            s[31] = *b as u8;
            s
        }
        Value::String(x) => sha(x.as_bytes()),
        other => sha(&serde_jcs::to_vec(other).unwrap()),
    }
}

fn inner(l: &[u8; 32], r: &[u8; 32]) -> [u8; 32] {
    let mut b = [0u8; 65];
    b[0] = 1;
    b[1..33].copy_from_slice(l);
    b[33..].copy_from_slice(r);
    sha(&b)
}

// Returns the root this entity's record and path lead to.
fn open(entity: &Value, path: &[u8]) -> [u8; 32] {
    let attrs = entity["attrs"].as_object().expect("attrs");
    let mut names: Vec<&String> = attrs.keys().collect();
    names.sort();
    assert!(names.len() <= MAX_ATTRS);
    let mut block = vec![0u8; 1 + 32 * MAX_ATTRS];
    for (i, n) in names.iter().enumerate() {
        block[0] |= 1 << i;
        block[1 + 32 * i..1 + 32 * (i + 1)].copy_from_slice(&slot(&attrs[*n]));
    }
    let mut ancs: Vec<[u8; 32]> =
        entity["parents"].as_array().expect("parents").iter().map(|p| sha(uid_text(p).unwrap().as_bytes())).collect();
    ancs.sort();
    assert!(ancs.len() <= MAX_ANC);
    let mut anc_block = vec![0u8; 32 * MAX_ANC];
    for (i, a) in ancs.iter().enumerate() {
        anc_block[32 * i..32 * (i + 1)].copy_from_slice(a);
    }
    let mut leaf = Vec::with_capacity(96);
    leaf.extend_from_slice(&sha(uid_text(&entity["uid"]).unwrap().as_bytes()));
    leaf.extend_from_slice(&sha(&block));
    leaf.extend_from_slice(&sha(&anc_block));
    let mut h = sha(&leaf);
    for step in path.chunks(33) {
        let sib: [u8; 32] = step[..32].try_into().unwrap();
        h = if step[32] == 1 { inner(&sib, &h) } else { inner(&h, &sib) };
    }
    h
}

fn main() {
    let mut marks: Vec<u64> = vec![env::cycle_count()];
    let mut input = Vec::new();
    env::stdin().read_to_end(&mut input).unwrap();
    let include_reasons = input[0] & 1 == 1;
    let mut at = 1;
    let policies_bytes = take(&input, &mut at);
    let schema_bytes = take(&input, &mut at);
    let request_bytes = take(&input, &mut at);
    let root: [u8; 32] = take(&input, &mut at).try_into().unwrap();
    let p_json = take(&input, &mut at);
    let p_path = take(&input, &mut at);
    let r_json = take(&input, &mut at);
    let r_path = take(&input, &mut at);
    marks.push(env::cycle_count());

    // Open the two records against the root.
    let p: Value = serde_json::from_slice(p_json).expect("principal");
    let r: Value = serde_json::from_slice(r_json).expect("resource");
    assert_eq!(open(&p, p_path), root, "principal opening");
    assert_eq!(open(&r, r_path), root, "resource opening");
    marks.push(env::cycle_count());

    let policies = PolicySet::from_str(core::str::from_utf8(policies_bytes).unwrap()).expect("policy set");
    marks.push(env::cycle_count());

    let entities = Entities::from_json_value(Value::Array(vec![p.clone(), r.clone()]), None).expect("entities");
    marks.push(env::cycle_count());

    // The request must name the opened entities.
    let req: Value = serde_json::from_slice(request_bytes).expect("request");
    let s = |v: &Value, k: &str| v[k].as_str().expect("field").to_owned();
    assert_eq!(uid_text(&req["subject"]), uid_text(&p["uid"]), "principal is the opened one");
    assert_eq!(uid_text(&req["resource"]), uid_text(&r["uid"]), "resource is the opened one");
    let action_name = s(&req["action"], "name");
    let action = if action_name.contains("::") { EntityUid::from_str(&action_name).expect("action") } else { uid("Action", &action_name) };
    let principal = uid(&s(&req["subject"], "type"), &s(&req["subject"], "id"));
    let resource = uid(&s(&req["resource"], "type"), &s(&req["resource"], "id"));
    let mut ctx = req.get("context").cloned().unwrap_or(serde_json::json!({}));
    let nonce = ctx.get("nonce").and_then(|n| n.as_str()).unwrap_or("").to_owned();
    if let Some(o) = ctx.as_object_mut() {
        o.remove("nonce");
        o.remove("epoch");
    }
    let request = Request::new(principal, action, resource, Context::from_json_value(ctx, None).expect("context"), None).expect("request");
    marks.push(env::cycle_count());

    let response = Authorizer::new().is_authorized(&request, &policies, &entities);
    let decision = response.decision() == Decision::Allow;
    let reasons = include_reasons.then(|| {
        let mut x: Vec<String> = response.diagnostics().reason().map(|p| p.to_string()).collect();
        x.sort();
        x
    });
    marks.push(env::cycle_count());

    env::commit(&Journal {
        v: 0,
        request_hash: sha(request_bytes),
        policy_hash: sha(policies_bytes),
        schema_hash: sha(schema_bytes),
        entities: EntityCommitment { kind: KIND.into(), root },
        nonce,
        decision,
        reasons,
    });
    marks.push(env::cycle_count());

    let mut out = Vec::with_capacity(marks.len() * 8);
    for m in &marks {
        out.extend_from_slice(&m.to_le_bytes());
    }
    env::stdout().write_all(&out).unwrap();
}
