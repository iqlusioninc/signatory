//! NIST P-256

#[cfg(feature = "test-vectors")]
pub mod test_vectors;

pub use p256::NistP256;

/// NIST P-256 public key
pub type PublicKey = super::PublicKey<NistP256>;

/// NIST P-256 secret key
pub type SecretKey = super::SecretKey<NistP256>;

/// NIST P-256 ASN.1 signature
pub type Asn1Signature = super::Asn1Signature<NistP256>;

/// NIST P-256 fixed signature
pub type FixedSignature = super::FixedSignature<NistP256>;
