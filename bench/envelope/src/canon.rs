//! Canonical bytes and hashes. See docs/protocol.md, "Journal".

use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// JCS (RFC 8785) bytes of a JSON value.
pub fn jcs(value: &Value) -> Vec<u8> {
    serde_jcs::to_vec(value).expect("JSON value serializes")
}

/// The canonical request: only the four AuthZEN members, in JCS.
pub fn canonical_request(request: &Value) -> Vec<u8> {
    let mut o = serde_json::Map::new();
    for k in ["subject", "action", "resource", "context"] {
        if let Some(v) = request.get(k) {
            o.insert(k.into(), v.clone());
        }
    }
    jcs(&Value::Object(o))
}

pub fn request_hash(request: &Value) -> [u8; 32] {
    sha256(&canonical_request(request))
}

/// The canonical Cedar text of an entity UID in Cedar's entity JSON:
/// `Type::"id"`. Accepts `{"type","id"}` and `{"__entity":{"type","id"}}`.
pub fn uid_text(uid: &Value) -> Option<String> {
    let u = uid.get("__entity").unwrap_or(uid);
    let t = u.get("type")?.as_str()?;
    let id = u.get("id")?.as_str()?;
    let mut esc = String::with_capacity(id.len());
    for c in id.chars() {
        match c {
            '"' => esc.push_str("\\\""),
            '\\' => esc.push_str("\\\\"),
            c => esc.push(c),
        }
    }
    Some(format!("{t}::\"{esc}\""))
}

/// Entities sorted by UID text, as UTF-8 bytes. Errors on a missing UID.
pub fn sorted_entities(entities: &Value) -> Result<Vec<Value>, String> {
    let arr = entities.as_array().ok_or("entities must be a JSON array")?;
    let mut keyed = Vec::with_capacity(arr.len());
    for e in arr {
        let k = e.get("uid").and_then(uid_text).ok_or("entity without a uid")?;
        keyed.push((k, e.clone()));
    }
    keyed.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    Ok(keyed.into_iter().map(|(_, e)| e).collect())
}

/// `sha256-jcs-v0`: JCS of the entity array sorted by UID.
pub fn canonical_entities(entities: &Value) -> Result<Vec<u8>, String> {
    Ok(jcs(&Value::Array(sorted_entities(entities)?)))
}

pub fn hex_hash(h: &[u8; 32]) -> String {
    format!("sha256:{}", hex::encode(h))
}

pub fn parse_hex_hash(s: &str) -> Option<[u8; 32]> {
    let h = s.strip_prefix("sha256:")?;
    hex::decode(h).ok()?.try_into().ok()
}
