//! `merkle-sha256-record-v0`: an entity commitment a circuit can open cheaply.
//!
//! `merkle-sha256-v0` hashes each entity's JSON. A circuit would have to parse
//! JSON to use it. Here each entity is a fixed-shape record, so a circuit
//! opens a leaf and reads attributes and ancestors from fixed positions. The
//! ancestors are the transitive closure, computed by whoever publishes the
//! root, so `in` is one scan and a deny can be proved as well as a permit.
//!
//! uid_h   = SHA-256(UTF-8 of `Type::"id"`)
//! slot    = 32 bytes for one value:
//!             entity reference   its uid_h
//!             integer            24 zero bytes, then the i64 big-endian
//!             boolean            31 zero bytes, then 0 or 1
//!             string             SHA-256 of its UTF-8
//!             set                SHA-256 of a set block
//!             record             SHA-256 of a record block
//!             anything else      SHA-256 of its JCS bytes
//! set block    = MAX_SET element slots in ascending byte order, unused zero
//! record block = one presence byte, bit i set when attribute i is present,
//!                then MAX_ATTRS slots in the layout's attribute order,
//!                absent or unused slots zero
//! ancs    = MAX_ANC slots of ancestor uid_h in ascending byte order, unused
//!           slots zero
//! leaf    = SHA-256(uid_h || SHA-256(record block of attrs) || SHA-256(ancs))
//! tree    = as merkle-sha256-v0: count leaf first, entities in UID order
//!
//! The layout says, per entity type, which attributes exist, in what order,
//! and which are sets or records. It comes from the schema. Without a layout
//! an entity's attributes are the ones present, in name order, with sets and
//! records hashed as opaque JCS values.

use crate::canon::{jcs, sha256, sorted_entities, uid_text};
use crate::merkle::{merkle_path, merkle_root, Step};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const ENTITIES_MERKLE_SHA256_RECORD_V0: &str = "merkle-sha256-record-v0";
pub const MAX_ATTRS: usize = 8;
pub const MAX_ANC: usize = 16;
pub const MAX_SET: usize = 8;

/// How one value is laid out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One slot, encoded by the value's own JSON shape.
    Leaf,
    Set(Box<Kind>),
    Record(Vec<(String, Kind)>),
}

/// Attribute layouts by entity type name.
pub type Layouts = BTreeMap<String, Vec<(String, Kind)>>;

pub fn uid_hash(uid_text: &str) -> [u8; 32] {
    sha256(uid_text.as_bytes())
}

fn is_entity_ref(v: &Value) -> bool {
    let u = v.get("__entity").unwrap_or(v);
    matches!(u.as_object(), Some(o) if o.len() == 2 && o.get("type").map_or(false, Value::is_string) && o.get("id").map_or(false, Value::is_string))
}

pub fn value_slot(v: &Value, kind: &Kind) -> Result<[u8; 32], String> {
    Ok(match kind {
        Kind::Set(elem) => sha256(&set_block(v, elem)?.concat()),
        Kind::Record(layout) => {
            let (present, slots) = record_block(v.as_object().ok_or("record attribute is not an object")?, layout)?;
            sha256(&block_bytes(present, &slots))
        }
        Kind::Leaf => {
            if is_entity_ref(v) {
                return Ok(uid_hash(&uid_text(v).unwrap()));
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
    })
}

pub fn set_block(v: &Value, elem: &Kind) -> Result<[[u8; 32]; MAX_SET], String> {
    let arr = v.as_array().ok_or("set attribute is not an array")?;
    let mut hs: BTreeSet<[u8; 32]> = BTreeSet::new();
    for e in arr {
        hs.insert(value_slot(e, elem)?);
    }
    if hs.len() > MAX_SET {
        return Err(format!("a set has {} elements, more than {MAX_SET}", hs.len()));
    }
    let mut out = [[0u8; 32]; MAX_SET];
    for (i, h) in hs.into_iter().enumerate() {
        out[i] = h;
    }
    Ok(out)
}

pub fn record_block(attrs: &Map<String, Value>, layout: &[(String, Kind)]) -> Result<(u8, [[u8; 32]; MAX_ATTRS]), String> {
    if layout.len() > MAX_ATTRS {
        return Err(format!("a record has {} attributes, more than {MAX_ATTRS}", layout.len()));
    }
    if let Some(extra) = attrs.keys().find(|k| !layout.iter().any(|(n, _)| n == *k)) {
        return Err(format!("attribute `{extra}` is not in the schema"));
    }
    let mut present = 0u8;
    let mut slots = [[0u8; 32]; MAX_ATTRS];
    for (i, (name, kind)) in layout.iter().enumerate() {
        if let Some(v) = attrs.get(name) {
            present |= 1 << i;
            slots[i] = value_slot(v, kind)?;
        }
    }
    Ok((present, slots))
}

pub fn block_bytes(present: u8, slots: &[[u8; 32]; MAX_ATTRS]) -> Vec<u8> {
    let mut a = Vec::with_capacity(1 + 32 * MAX_ATTRS);
    a.push(present);
    slots.iter().for_each(|s| a.extend_from_slice(s));
    a
}

/// One entity as the circuit sees it.
#[derive(Clone, Debug)]
pub struct Record {
    pub uid_text: String,
    pub uid_h: [u8; 32],
    pub present: u8,
    pub attrs: [[u8; 32]; MAX_ATTRS],
    pub ancestors: [[u8; 32]; MAX_ANC],
}

impl Record {
    pub fn leaf(&self) -> [u8; 32] {
        let mut l = Vec::with_capacity(96);
        l.extend_from_slice(&self.uid_h);
        l.extend_from_slice(&sha256(&block_bytes(self.present, &self.attrs)));
        l.extend_from_slice(&sha256(&self.ancestors.concat()));
        sha256(&l)
    }
}

/// The committed store: records in UID order, the tree's leaves and its root.
pub struct RecordStore {
    pub records: Vec<Record>,
    pub json: BTreeMap<String, Value>,
    leaves: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

impl RecordStore {
    pub fn new(entities: &Value, layouts: &Layouts) -> Result<RecordStore, String> {
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
        let mut json = BTreeMap::new();
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

            let empty = Map::new();
            let attrs_json = e["attrs"].as_object().unwrap_or(&empty);
            let ty = e["uid"].get("__entity").unwrap_or(&e["uid"])["type"].as_str().unwrap_or("");
            let fallback: Vec<(String, Kind)>;
            let layout = match layouts.get(ty) {
                Some(l) => l,
                None => {
                    let mut names: Vec<&String> = attrs_json.keys().collect();
                    names.sort();
                    fallback = names.into_iter().map(|n| (n.clone(), Kind::Leaf)).collect();
                    &fallback
                }
            };
            let (present, attrs) = record_block(attrs_json, layout).map_err(|m| format!("{uid}: {m}"))?;
            json.insert(uid.clone(), Value::Object(attrs_json.clone()));
            records.push(Record { uid_h: uid_hash(&uid), uid_text: uid, present, attrs, ancestors });
        }
        let mut leaves = vec![sha256(&(records.len() as u64).to_be_bytes())];
        leaves.extend(records.iter().map(Record::leaf));
        let root = merkle_root(&leaves);
        Ok(RecordStore { records, json, leaves, root })
    }

    /// The record for a UID and its inclusion path.
    pub fn open(&self, uid_text: &str) -> Option<(&Record, Vec<Step>)> {
        let i = self.records.iter().position(|r| r.uid_text == uid_text)?;
        Some((&self.records[i], merkle_path(&self.leaves, i + 1)))
    }
}
