//! The guest: the Cedar authorizer plus a thin shell.
//!
//! Inputs are data, so the image ID does not change when policy changes
//! (decision 0007). The guest hashes the exact bytes it parses. Canonical
//! form is the authority's job: a verifier allowlists hashes of canonical
//! bytes, so non-canonical input only yields a hash nobody accepts.

use std::io::{Read, Write};
use std::str::FromStr;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet,
    Request, Schema,
};
use envelope::{EntityCommitment, Journal, ENTITIES_SHA256_JCS_V0};
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};

fn take<'a>(buf: &'a [u8], at: &mut usize) -> &'a [u8] {
    let n = u32::from_le_bytes(buf[*at..*at + 4].try_into().unwrap()) as usize;
    let out = &buf[*at + 4..*at + 4 + n];
    *at += 4 + n;
    out
}

fn sha(bytes: &[u8]) -> [u8; 32] {
    let d = Impl::hash_bytes(bytes);
    d.as_bytes().try_into().unwrap()
}

fn uid(type_name: &str, id: &str) -> EntityUid {
    EntityUid::from_type_name_and_id(
        EntityTypeName::from_str(type_name).expect("entity type"),
        EntityId::new(id),
    )
}

fn main() {
    let mut marks: Vec<u64> = Vec::with_capacity(8);
    marks.push(env::cycle_count());

    // 1. Read. One frame: flags byte, then five length-prefixed fields.
    let mut input = Vec::new();
    env::stdin().read_to_end(&mut input).unwrap();
    let include_reasons = input[0] & 1 == 1;
    let mut at = 1;
    let policies_bytes = take(&input, &mut at);
    let schema_bytes = take(&input, &mut at);
    let entities_bytes = take(&input, &mut at);
    let request_bytes = take(&input, &mut at);
    marks.push(env::cycle_count());

    // 2. Parse the policy set.
    let policies =
        PolicySet::from_str(core::str::from_utf8(policies_bytes).unwrap()).expect("policy set");
    marks.push(env::cycle_count());

    // 3. Parse the schema, if one was given.
    let schema = if schema_bytes.is_empty() {
        None
    } else {
        let (s, _warnings) =
            Schema::from_cedarschema_str(core::str::from_utf8(schema_bytes).unwrap())
                .expect("schema");
        Some(s)
    };
    marks.push(env::cycle_count());

    // 4. Parse the entities. Cedar computes the transitive closure here.
    let entities =
        Entities::from_json_str(core::str::from_utf8(entities_bytes).unwrap(), schema.as_ref())
            .expect("entities");
    marks.push(env::cycle_count());

    // 5. Parse the AuthZEN request and map it to a Cedar request.
    let req: serde_json::Value = serde_json::from_slice(request_bytes).expect("request json");
    let s = |v: &serde_json::Value, k: &str| v[k].as_str().expect("string field").to_owned();
    let principal = uid(&s(&req["subject"], "type"), &s(&req["subject"], "id"));
    let resource = uid(&s(&req["resource"], "type"), &s(&req["resource"], "id"));
    let action_name = s(&req["action"], "name");
    let action = if action_name.contains("::") {
        EntityUid::from_str(&action_name).expect("action uid")
    } else {
        uid("Action", &action_name)
    };
    // The nonce and epoch are for the verifier, not for the policy.
    let mut ctx = req.get("context").cloned().unwrap_or(serde_json::json!({}));
    let nonce = ctx.get("nonce").and_then(|n| n.as_str()).unwrap_or("").to_owned();
    if let Some(o) = ctx.as_object_mut() {
        o.remove("nonce");
        o.remove("epoch");
    }
    let context =
        Context::from_json_value(ctx, schema.as_ref().map(|s| (s, &action))).expect("context");
    let request =
        Request::new(principal, action, resource, context, schema.as_ref()).expect("request");
    marks.push(env::cycle_count());

    // 6. Authorize.
    let response = Authorizer::new().is_authorized(&request, &policies, &entities);
    let decision = response.decision() == Decision::Allow;
    let reasons = if include_reasons {
        let mut r: Vec<String> =
            response.diagnostics().reason().map(|p| p.to_string()).collect();
        r.sort();
        Some(r)
    } else {
        None
    };
    marks.push(env::cycle_count());

    // 7. Hash what the journal names and commit it.
    let journal = Journal {
        v: 0,
        request_hash: sha(request_bytes),
        policy_hash: sha(policies_bytes),
        schema_hash: if schema_bytes.is_empty() { [0u8; 32] } else { sha(schema_bytes) },
        entities: EntityCommitment {
            kind: ENTITIES_SHA256_JCS_V0.into(),
            root: sha(entities_bytes),
        },
        nonce,
        decision,
        reasons,
    };
    env::commit(&journal);
    marks.push(env::cycle_count());

    // Cycle marks go to stdout, outside the journal.
    let mut out = Vec::with_capacity(marks.len() * 8);
    for m in &marks {
        out.extend_from_slice(&m.to_le_bytes());
    }
    env::stdout().write_all(&out).unwrap();
}
