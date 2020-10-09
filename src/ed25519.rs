//! Ed25519: Schnorr signatures using the twisted Edwards form of Curve25519
//!
//! Described in RFC 8032: <https://tools.ietf.org/html/rfc8032>
//!
//! This module contains two convenience methods for signing and verifying
//! Ed25519 signatures which work with any signer or verifier.

mod public_key;
mod seed;

#[cfg(feature = "test-vectors")]
#[macro_use]
mod test_macros;

/// RFC 8032 Ed25519 test vectors
#[cfg(feature = "test-vectors")]
mod test_vectors;

#[cfg(feature = "test-vectors")]
pub use self::test_vectors::TEST_VECTORS;
pub use self::{
    public_key::{PublicKey, PUBLIC_KEY_SIZE},
    seed::{Seed, SEED_SIZE},
};

// Import `Signature` type from the `ed25519` crate
pub use ed25519::{Signature, SIGNATURE_LENGTH as SIGNATURE_SIZE};
