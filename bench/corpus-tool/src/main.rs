//! Canonicalize corpus/raw into corpus/sets, run the native Cedar evaluator
//! as the oracle, and write corpus/MANIFEST.json.
//!
//! Usage: corpus-tool <corpus dir>

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use cedar_policy::{
    Authorizer, Context, Decision, Entities, EntityId, EntityTypeName, EntityUid, PolicySet,
    Request, Schema,
};
use cedar_policy_formatter::{policies_str_to_pretty, Config};
use envelope::{canonical_entities, canonical_request, entities_merkle_root, hex_hash, sha256};
use serde_json::{json, Value};

fn uid(t: &str, id: &str) -> EntityUid {
    EntityUid::from_type_name_and_id(EntityTypeName::from_str(t).expect("type"), EntityId::new(id))
}

/// The AuthZEN to Cedar mapping. Written apart from the guest's on purpose:
/// the oracle should not share code with what it checks.
fn to_cedar(req: &Value, schema: Option<&Schema>) -> Request {
    let f = |p: &str| req.pointer(p).and_then(Value::as_str).expect(p).to_owned();
    let principal = uid(&f("/subject/type"), &f("/subject/id"));
    let resource = uid(&f("/resource/type"), &f("/resource/id"));
    let name = f("/action/name");
    let action = if name.contains("::") { EntityUid::from_str(&name).expect("action") } else { uid("Action", &name) };
    let mut ctx = req.get("context").cloned().unwrap_or(json!({}));
    if let Some(o) = ctx.as_object_mut() {
        o.remove("nonce");
        o.remove("epoch");
    }
    let context = Context::from_json_value(ctx, schema.map(|s| (s, &action))).expect("context");
    Request::new(principal, action, resource, context, schema).expect("request")
}

fn main() {
    let corpus = std::env::args().nth(1).expect("corpus dir");
    let corpus = Path::new(&corpus);
    let mut manifest = BTreeMap::new();
    let mut names: Vec<_> = fs::read_dir(corpus.join("raw")).unwrap().map(|e| e.unwrap().file_name()).collect();
    names.sort();
    let mut total = 0;
    for name in names {
        let raw = corpus.join("raw").join(&name);
        let out = corpus.join("sets").join(&name);
        fs::create_dir_all(&out).unwrap();

        let policies_raw = fs::read_to_string(raw.join("policies.cedar")).unwrap();
        let policies_text = policies_str_to_pretty(&policies_raw, &Config::default()).expect("format");
        let schema_text = fs::read_to_string(raw.join("schema.cedarschema")).unwrap();
        let entities_json: Value = serde_json::from_slice(&fs::read(raw.join("entities.json")).unwrap()).unwrap();
        let entities_bytes = canonical_entities(&entities_json).expect("entities");
        let requests: Vec<Value> = serde_json::from_slice(&fs::read(raw.join("requests.json")).unwrap()).unwrap();

        let policies = PolicySet::from_str(&policies_text).expect("policies");
        // Formatting must not change meaning.
        assert_eq!(
            PolicySet::from_str(&policies_raw).unwrap().policies().count(),
            policies.policies().count()
        );
        let (schema, _) = Schema::from_cedarschema_str(&schema_text).expect("schema");
        let entities_str = std::str::from_utf8(&entities_bytes).unwrap();
        // Some upstream stores do not conform to their own schema. Upstream
        // authorizes those without a schema, and so do we. The manifest says which.
        let (entities, uses_schema, schema_error) = match Entities::from_json_str(entities_str, Some(&schema)) {
            Ok(e) => (e, true, None),
            Err(err) => {
                let first = err.to_string().lines().next().unwrap_or("").to_owned();
                (Entities::from_json_str(entities_str, None).expect("entities"), false, Some(first))
            }
        };
        let schema_opt = if uses_schema { Some(&schema) } else { None };

        let mut lines = String::new();
        let mut request_hashes = Vec::new();
        for r in &requests {
            let req = &r["request"];
            let resp = Authorizer::new().is_authorized(&to_cedar(req, schema_opt), &policies, &entities);
            let decision = resp.decision() == Decision::Allow;
            let mut reasons: Vec<String> = resp.diagnostics().reason().map(|p| p.to_string()).collect();
            reasons.sort();
            let errors: Vec<String> = resp.diagnostics().errors().map(|e| e.to_string()).collect();
            if let Some(up) = r.get("upstream_expectation").and_then(Value::as_bool) {
                assert_eq!(up, decision, "{:?} {}: oracle differs from upstream expectation", name, r["name"]);
            }
            let canon = canonical_request(req);
            let h = hex_hash(&sha256(&canon));
            request_hashes.push(h.clone());
            let line = json!({
                "name": r["name"],
                "request": serde_json::from_slice::<Value>(&canon).unwrap(),
                "request_hash": h,
                "oracle": { "decision": decision, "reasons": reasons, "errors": errors },
            });
            lines.push_str(&serde_json::to_string(&line).unwrap());
            lines.push('\n');
            total += 1;
        }

        fs::write(out.join("policies.cedar"), &policies_text).unwrap();
        fs::write(out.join("schema.cedarschema"), &schema_text).unwrap();
        fs::write(out.join("entities.json"), &entities_bytes).unwrap();
        fs::write(out.join("requests.jsonl"), &lines).unwrap();
        fs::copy(raw.join("NOTE"), out.join("NOTE")).unwrap();

        manifest.insert(
            name.to_string_lossy().into_owned(),
            json!({
                "policy_hash": hex_hash(&sha256(policies_text.as_bytes())),
                "schema_hash": hex_hash(&sha256(schema_text.as_bytes())),
                "entities_sha256_jcs_v0": hex_hash(&sha256(&entities_bytes)),
                "entities_merkle_sha256_v0": hex_hash(&entities_merkle_root(&entities_json).unwrap()),
                "requests_file_sha256": hex_hash(&sha256(lines.as_bytes())),
                "policies": policies.policies().count(),
                "entities": entities_json.as_array().unwrap().len(),
                "requests": requests.len(),
                "uses_schema": uses_schema,
                "schema_error": schema_error,
                "bytes": { "policies": policies_text.len(), "schema": schema_text.len(), "entities": entities_bytes.len() },
            }),
        );
    }
    let m = json!({ "cedar_policy": "4.14.1", "sets": manifest });
    fs::write(corpus.join("MANIFEST.json"), serde_json::to_string_pretty(&m).unwrap() + "\n").unwrap();
    println!("sets: {}  requests: {}", m["sets"].as_object().unwrap().len(), total);
}
