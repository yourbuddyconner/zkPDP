//! The verification procedure of docs/protocol.md. One check per step, stop
//! at the first failure.

use crate::canon::{parse_hex_hash, request_hash};
use crate::wire::{seal_decode, ProofObject};
use crate::Journal;
use serde_json::Value;

/// Checks a seal against an evaluator identity and a journal. One
/// implementation per proof system.
pub trait SealVerifier {
    fn verify(&self, format: &str, evaluator: &[u8; 32], journal: &Journal, seal: &[u8]) -> Result<(), String>;
}

/// What a verifier pins.
#[derive(Clone, Debug, Default)]
pub struct Pins {
    /// Accepted (format, evaluator identity) pairs.
    pub evaluators: Vec<(String, [u8; 32])>,
    pub policy_hashes: Vec<[u8; 32]>,
    /// Checked only when set.
    pub schema_hash: Option<[u8; 32]>,
    pub entity_roots: Vec<[u8; 32]>,
    /// The nonce this verifier issued. Checked only when set.
    pub nonce: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    Malformed(&'static str),
    Format,
    Evaluator,
    Seal(String),
    Request,
    Policy,
    Schema,
    Entities,
    Nonce,
    /// The top-level decision differs from the proven one.
    Decision,
}

/// Returns the proven decision, or the first check that failed.
pub fn verify(request: &Value, response: &Value, pins: &Pins, seals: &dyn SealVerifier) -> Result<bool, Refusal> {
    let proof = response.pointer("/context/proof").ok_or(Refusal::Malformed("no proof"))?;
    let proof: ProofObject =
        serde_json::from_value(proof.clone()).map_err(|_| Refusal::Malformed("proof object"))?;
    let journal = proof.journal.to_journal().ok_or(Refusal::Malformed("journal"))?;
    let evaluator = parse_hex_hash(&proof.evaluator).ok_or(Refusal::Malformed("evaluator"))?;
    let seal = seal_decode(&proof.seal).ok_or(Refusal::Malformed("seal"))?;

    // 1. Format.
    if !pins.evaluators.iter().any(|(f, _)| f == &proof.format) {
        return Err(Refusal::Format);
    }
    // 2. Evaluator. Compilation proofs are not accepted in version 0 of this crate.
    if !pins.evaluators.iter().any(|(f, e)| f == &proof.format && e == &evaluator) {
        return Err(Refusal::Evaluator);
    }
    // 3. Seal.
    seals.verify(&proof.format, &evaluator, &journal, &seal).map_err(Refusal::Seal)?;
    // 4. Request.
    if journal.request_hash != request_hash(request) {
        return Err(Refusal::Request);
    }
    // 5. Policy.
    if !pins.policy_hashes.contains(&journal.policy_hash) {
        return Err(Refusal::Policy);
    }
    // 6. Schema.
    if let Some(s) = pins.schema_hash {
        if journal.schema_hash != s {
            return Err(Refusal::Schema);
        }
    }
    // 7. Entities.
    if !pins.entity_roots.contains(&journal.entities.root) {
        return Err(Refusal::Entities);
    }
    // 8. Nonce.
    if let Some(n) = &pins.nonce {
        if &journal.nonce != n {
            return Err(Refusal::Nonce);
        }
    }
    // 9. The decision the response states must be the proven one.
    if response.get("decision").and_then(Value::as_bool) != Some(journal.decision) {
        return Err(Refusal::Decision);
    }
    Ok(journal.decision)
}
