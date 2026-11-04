//! The JSON form of the proof object inside `context.proof`.

use crate::canon::{hex_hash, parse_hex_hash};
use crate::{EntityCommitment, Journal};
use base64::Engine;
use serde::{Deserialize, Serialize};

pub const FORMAT_RISC0_COMPOSITE: &str = "risc0-composite-v0";
pub const FORMAT_RISC0_SUCCINCT: &str = "risc0-succinct-v0";
pub const FORMAT_RISC0_GROTH16: &str = "risc0-groth16-v0";
pub const FORMAT_NOIR_ULTRAHONK: &str = "noir-ultrahonk-v0";
pub const FORMAT_NOIR_ULTRAHONK_ZK: &str = "noir-ultrahonk-zk-v0";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntitiesWire {
    #[serde(rename = "type")]
    pub kind: String,
    pub root: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalWire {
    pub v: u32,
    pub request_hash: String,
    pub policy_hash: String,
    pub schema_hash: String,
    pub entities: EntitiesWire,
    pub nonce: String,
    pub decision: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reasons: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofObject {
    pub format: String,
    pub evaluator: String,
    pub journal: JournalWire,
    pub seal: String,
    #[serde(default)]
    pub compilation_proof: Option<Box<ProofObject>>,
}

impl From<&Journal> for JournalWire {
    fn from(j: &Journal) -> Self {
        JournalWire {
            v: j.v,
            request_hash: hex_hash(&j.request_hash),
            policy_hash: hex_hash(&j.policy_hash),
            schema_hash: hex_hash(&j.schema_hash),
            entities: EntitiesWire { kind: j.entities.kind.clone(), root: hex_hash(&j.entities.root) },
            nonce: j.nonce.clone(),
            decision: j.decision,
            reasons: j.reasons.clone(),
        }
    }
}

impl JournalWire {
    pub fn to_journal(&self) -> Option<Journal> {
        Some(Journal {
            v: self.v,
            request_hash: parse_hex_hash(&self.request_hash)?,
            policy_hash: parse_hex_hash(&self.policy_hash)?,
            schema_hash: parse_hex_hash(&self.schema_hash)?,
            entities: EntityCommitment {
                kind: self.entities.kind.clone(),
                root: parse_hex_hash(&self.entities.root)?,
            },
            nonce: self.nonce.clone(),
            decision: self.decision,
            reasons: self.reasons.clone(),
        })
    }
}

pub fn seal_encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn seal_decode(s: &str) -> Option<Vec<u8>> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s).ok()
}

/// An AuthZEN response carrying a proof.
pub fn response(journal: &Journal, format: &str, evaluator: &[u8; 32], seal: &[u8]) -> serde_json::Value {
    let proof = ProofObject {
        format: format.into(),
        evaluator: hex_hash(evaluator),
        journal: journal.into(),
        seal: seal_encode(seal),
        compilation_proof: None,
    };
    serde_json::json!({ "decision": journal.decision, "context": { "proof": proof } })
}
