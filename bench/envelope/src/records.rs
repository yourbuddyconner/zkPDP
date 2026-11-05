//! `merkle-sha256-record-v0`: an entity commitment a circuit can open cheaply.
//!
//! `merkle-sha256-v0` hashes each entity's JSON. A circuit would have to parse
//! JSON to use it. Here each entity is a fixed-shape record, so a circuit
//! opens two leaves, the principal's and the resource's, and reads attributes
//! and ancestors from fixed positions. The ancestors are the transitive
//! closure, computed by whoever publishes the root, so `in` is one scan and a
//! deny can be proved as well as a permit.
//!
//! uid_h   = SHA-256(UTF-8 of `Type::"id"`)
//! value   = 32 bytes: an entity reference is its uid_h, an integer is 24 zero
//!           bytes then the i64 big-endian, a boolean is 31 zero bytes then 0
//!           or 1, a string is SHA-256 of its UTF-8, anything else is SHA-256
//!           of its JCS bytes
//! attrs   = one count byte, then MAX_ATTRS value slots in attribute name
//!           order, unused slots zero
//! ancs    = MAX_ANC slots of ancestor uid_h in ascending byte order, unused
//!           slots zero
//! leaf    = SHA-256(uid_h || SHA-256(attrs) || SHA-256(ancs))
//! tree    = as merkle-sha256-v0: count leaf first, entities in UID order

use crate::canon::{jcs, sha256, sorted_entities, uid_text};
use crate::merkle::{merkle_path, merkle_root, Step};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const ENTITIES_MERKLE_SHA256_RECORD_V0: &str = "merkle-sha256-record-v0";
pub const MAX_ATTRS: usize = 8;
pub const MAX_ANC: usize = 8;

pub fn uid_hash(uid_text: &str) -> [u8; 32] {
    sha256(uid_text.as_bytes())
}

pub fn value_slot(v: &Value) -> [u8; 32] {
    if let Some(t) = v.get("__entity").and_then(|_| uid_text(v)) {
        return uid_hash(&t);
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
        Value::String(x) => sha256(x.as_bytes()),
        other => sha256(&jcs(other)),
    }
}

/// One entity as the circuit sees it.
#[derive(Clone, Debug)]
pub struct Record {
    pub uid_text: String,
    pub uid_h: [u8; 32],
    pub n_attrs: u8,
    pub attrs: [[u8; 32]; MAX_ATTRS],
    pub ancestors: [[u8; 32]; MAX_ANC],
}

impl Record {
    pub fn leaf(&self) -> [u8; 32] {
        let mut a = Vec::with_capacity(1 + 32 * MAX_ATTRS);
        a.push(self.n_attrs);
        self.attrs.iter().for_each(|s| a.extend_from_slice(s));
        let mut c = Vec::with_capacity(32 * MAX_ANC);
        self.ancestors.iter().for_each(|s| c.extend_from_slice(s));
        let mut l = Vec::with_capacity(96);
        l.extend_from_slice(&self.uid_h);
        l.extend_from_slice(&sha256(&a));
        l.extend_from_slice(&sha256(&c));
        sha256(&l)
    }
}

/// The committed store: records in UID order, the tree's leaves and its root.
pub struct RecordStore {
    pub records: Vec<Record>,
    leaves: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

impl RecordStore {
    pub fn new(entities: &Value) -> Result<RecordStore, String> {
        let sorted = sorted_entities(entities)?;
        let key = |e: &Value| e.get("uid").and_then(uid_text).unwrap();
        let parents: BTreeMap<String, Vec<String>> = sorted
            .iter()
            .map(|e| {
                let ps = e["parents"].as_array().map(|a| a.iter().filter_map(uid_text).collect()).unwrap_or_default();
                (key(e), ps)
            })
            .collect();
        let mut records = Vec::with_capacity(sorted.len());
        for e in &sorted {
            let uid = key(e);
            // Transitive closure of the parent relation.
            let mut seen = BTreeSet::new();
            let mut todo = parents[&uid].clone();
            while let Some(p) = todo.pop() {
                if seen.insert(p.clone()) {
                    todo.extend(parents.get(&p).cloned().unwrap_or_default());
                }
            }
            if seen.len() > MAX_ANC {
                return Err(format!("{uid} has {} ancestors, more than {MAX_ANC}", seen.len()));
            }
            let mut hs: Vec<[u8; 32]> = seen.iter().map(|a| uid_hash(a)).collect();
            hs.sort();
            let mut ancestors = [[0u8; 32]; MAX_ANC];
            ancestors[..hs.len()].copy_from_slice(&hs);

            let empty = serde_json::Map::new();
            let attrs_json = e["attrs"].as_object().unwrap_or(&empty);
            if attrs_json.len() > MAX_ATTRS {
                return Err(format!("{uid} has more than {MAX_ATTRS} attributes"));
            }
            let mut names: Vec<&String> = attrs_json.keys().collect();
            names.sort();
            let mut attrs = [[0u8; 32]; MAX_ATTRS];
            for (i, n) in names.iter().enumerate() {
                attrs[i] = value_slot(&attrs_json[*n]);
            }
            records.push(Record { uid_h: uid_hash(&uid), uid_text: uid, n_attrs: names.len() as u8, attrs, ancestors });
        }
        let mut leaves = vec![sha256(&(records.len() as u64).to_be_bytes())];
        leaves.extend(records.iter().map(Record::leaf));
        let root = merkle_root(&leaves);
        Ok(RecordStore { records, leaves, root })
    }

    /// The record for a UID and its inclusion path.
    pub fn open(&self, uid_text: &str) -> Option<(&Record, Vec<Step>)> {
        let i = self.records.iter().position(|r| r.uid_text == uid_text)?;
        Some((&self.records[i], merkle_path(&self.leaves, i + 1)))
    }
}
