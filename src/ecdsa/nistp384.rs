//! NIST P-384

#[cfg(feature = "test-vectors")]
pub mod test_vectors;

pub use p384::NistP384;

/// NIST P-384 public key
pub type PublicKey = super::PublicKey<NistP384>;

/// NIST P-384 secret key
pub type SecretKey = super::SecretKey<NistP384>;

/// NIST P-384 ASN.1 signature
pub type Asn1Signature = super::Asn1Signature<NistP384>;

/// NIST P-384 fixed signature
pub type FixedSignature = super::FixedSignature<NistP384>;
