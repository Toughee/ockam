use crate::util::*;
use bls12_381_plus::Scalar;
use ff::Field;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use subtle::CtOption;

/// A message that is signed into a signature
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SignatureBlinding(pub(crate) Scalar);

impl SignatureBlinding {
    /// The number of bytes in a message
    pub const BYTES: usize = 32;

    /// Hash arbitrary data to a signature blinding to be signed into BBS+
    pub fn hash<B: AsRef<[u8]>>(data: B) -> Self {
        Self(hash_to_scalar(data))
    }

    /// Generate a random signature blinding
    pub fn random(rng: impl RngCore) -> Self {
        Self(Scalar::random(rng))
    }

    /// Get the byte sequence that represents this signature blinding
    pub fn to_bytes(&self) -> [u8; Self::BYTES] {
        scalar_to_bytes(self.0)
    }

    /// Convert a big-endian representation of the signature blinding
    pub fn from_bytes(bytes: &[u8; Self::BYTES]) -> CtOption<Self> {
        scalar_from_bytes(bytes).map(|s| Self(s))
    }
}
