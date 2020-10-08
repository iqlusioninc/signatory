//! secp256k1

#[cfg(feature = "test-vectors")]
pub mod test_vectors;

pub use k256::Secp256k1;

/// secp256k1 public key
pub type PublicKey = super::PublicKey<Secp256k1>;

/// secp256k1 secret key
pub type SecretKey = super::SecretKey<Secp256k1>;

/// secp256k1 ASN.1 signature
pub type Asn1Signature = super::Asn1Signature<Secp256k1>;

/// secp256k1 fixed signature
pub type FixedSignature = super::FixedSignature<Secp256k1>;
