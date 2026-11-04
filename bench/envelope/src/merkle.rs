//! `merkle-sha256-v0`: the entity commitment that supports inclusion proofs.
//!
//! Leaf 0 is SHA-256 of the entity count as 8 big-endian bytes. Leaf i+1 is
//! SHA-256 of the JCS bytes of the i-th entity, sorted by UID. An inner node
//! is SHA-256 of 0x01 || left || right. A node without a sibling moves up
//! unchanged. JCS of an entity starts with `{`, so a leaf preimage can never
//! be read as an inner node preimage.

use crate::canon::{jcs, sha256, sorted_entities};
use serde_json::Value;

fn inner(l: &[u8; 32], r: &[u8; 32]) -> [u8; 32] {
    let mut b = [0u8; 65];
    b[0] = 1;
    b[1..33].copy_from_slice(l);
    b[33..].copy_from_slice(r);
    sha256(&b)
}

pub fn entity_leaves(entities: &Value) -> Result<Vec<[u8; 32]>, String> {
    let sorted = sorted_entities(entities)?;
    let mut leaves = Vec::with_capacity(sorted.len() + 1);
    leaves.push(sha256(&(sorted.len() as u64).to_be_bytes()));
    leaves.extend(sorted.iter().map(|e| sha256(&jcs(e))));
    Ok(leaves)
}

pub fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    let mut level = leaves.to_vec();
    while level.len() > 1 {
        level = level
            .chunks(2)
            .map(|c| if c.len() == 2 { inner(&c[0], &c[1]) } else { c[0] })
            .collect();
    }
    level[0]
}

/// One step of an inclusion path: the sibling and whether it is on the left.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    pub sibling: [u8; 32],
    pub sibling_is_left: bool,
}

pub fn merkle_path(leaves: &[[u8; 32]], mut index: usize) -> Vec<Step> {
    let mut path = Vec::new();
    let mut level = leaves.to_vec();
    while level.len() > 1 {
        let sib = index ^ 1;
        if sib < level.len() {
            path.push(Step { sibling: level[sib], sibling_is_left: sib < index });
        }
        level = level
            .chunks(2)
            .map(|c| if c.len() == 2 { inner(&c[0], &c[1]) } else { c[0] })
            .collect();
        index /= 2;
    }
    path
}

pub fn merkle_verify(leaf: &[u8; 32], path: &[Step], root: &[u8; 32]) -> bool {
    let mut h = *leaf;
    for s in path {
        h = if s.sibling_is_left { inner(&s.sibling, &h) } else { inner(&h, &s.sibling) };
    }
    &h == root
}

pub fn entities_merkle_root(entities: &Value) -> Result<[u8; 32], String> {
    Ok(merkle_root(&entity_leaves(entities)?))
}
