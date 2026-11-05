//! The proof-carrying AuthZEN response: journal, canonical hashes, entity
//! commitment and the verification procedure. See docs/protocol.md.
//!
//! Without the `std` feature this crate is only the journal type, which is
//! what a guest commits.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

pub const ENTITIES_SHA256_JCS_V0: &str = "sha256-jcs-v0";
pub const ENTITIES_MERKLE_SHA256_V0: &str = "merkle-sha256-v0";

/// What the entity store is pinned by.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityCommitment {
    pub kind: String,
    pub root: [u8; 32],
}

/// The public output of one decision. Field order is the order of public
/// inputs for the circuit backend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Journal {
    pub v: u32,
    pub request_hash: [u8; 32],
    pub policy_hash: [u8; 32],
    /// All zero when no schema was used.
    pub schema_hash: [u8; 32],
    pub entities: EntityCommitment,
    pub nonce: String,
    pub decision: bool,
    pub reasons: Option<Vec<String>>,
}

#[cfg(feature = "std")]
mod canon;
#[cfg(feature = "std")]
mod merkle;
#[cfg(feature = "std")]
mod records;
#[cfg(feature = "std")]
mod verify;
#[cfg(feature = "std")]
mod wire;

#[cfg(feature = "std")]
pub use canon::*;
#[cfg(feature = "std")]
pub use merkle::*;
#[cfg(feature = "std")]
pub use records::*;
#[cfg(feature = "std")]
pub use verify::*;
#[cfg(feature = "std")]
pub use wire::*;
