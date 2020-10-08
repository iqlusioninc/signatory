//! ECDSA P-256 provider for the *ring* crate

pub use signatory::ecdsa::nistp256::{Asn1Signature, FixedSignature, NistP256};

use super::signer::EcdsaSigner;
use ring::signature::{
    UnparsedPublicKey, ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING,
    ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING,
};
use signatory::{
    public_key::PublicKeyed,
    signature::{self, Signature},
};

#[cfg(feature = "std")]
use ring::rand::SystemRandom;
#[cfg(feature = "std")]
use signatory::encoding::{
    self,
    pkcs8::{self, FromPkcs8, GeneratePkcs8},
};

/// NIST P-256 public key
pub type PublicKey = signatory::ecdsa::PublicKey<NistP256>;

/// NIST P-256 ECDSA signer
pub struct Signer<S: Signature>(EcdsaSigner<S>);

#[cfg(feature = "std")]
impl FromPkcs8 for Signer<Asn1Signature> {
    /// Create a new ECDSA signer which produces fixed-width signatures from a PKCS#8 keypair
    fn from_pkcs8<K: AsRef<[u8]>>(secret_key: K) -> Result<Self, encoding::Error> {
        Ok(Signer(EcdsaSigner::from_pkcs8(
            &ECDSA_P256_SHA256_ASN1_SIGNING,
            secret_key.as_ref(),
        )?))
    }
}

#[cfg(feature = "std")]
impl FromPkcs8 for Signer<FixedSignature> {
    /// Create a new ECDSA signer which produces fixed-width signatures from a PKCS#8 keypair
    fn from_pkcs8<K: AsRef<[u8]>>(secret_key: K) -> Result<Self, encoding::Error> {
        Ok(Signer(EcdsaSigner::from_pkcs8(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            secret_key.as_ref(),
        )?))
    }
}

#[cfg(feature = "std")]
impl GeneratePkcs8 for Signer<Asn1Signature> {
    /// Randomly generate a P-256 **PKCS#8** keypair
    fn generate_pkcs8() -> Result<pkcs8::SecretKey, encoding::Error> {
        let keypair = ring::signature::EcdsaKeyPair::generate_pkcs8(
            &ECDSA_P256_SHA256_ASN1_SIGNING,
            &SystemRandom::new(),
        )
        .unwrap();

        pkcs8::SecretKey::from_bytes(keypair.as_ref())
    }
}

#[cfg(feature = "std")]
impl GeneratePkcs8 for Signer<FixedSignature> {
    /// Randomly generate a P-256 **PKCS#8** keypair
    fn generate_pkcs8() -> Result<pkcs8::SecretKey, encoding::Error> {
        let keypair = ring::signature::EcdsaKeyPair::generate_pkcs8(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            &SystemRandom::new(),
        )
        .unwrap();

        pkcs8::SecretKey::from_bytes(keypair.as_ref())
    }
}

impl<S> PublicKeyed<PublicKey> for Signer<S>
where
    S: Signature + Send + Sync,
{
    fn public_key(&self) -> Result<PublicKey, signature::Error> {
        PublicKey::from_bytes(self.0.public_key()).map_err(|_| signature::Error::new())
    }
}

impl signature::Signer<Asn1Signature> for Signer<Asn1Signature> {
    fn try_sign(&self, msg: &[u8]) -> Result<Asn1Signature, signature::Error> {
        self.0.sign(msg)
    }
}

impl signature::Signer<FixedSignature> for Signer<FixedSignature> {
    fn try_sign(&self, msg: &[u8]) -> Result<FixedSignature, signature::Error> {
        self.0.sign(msg)
    }
}

/// NIST P-256 ECDSA verifier
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verifier(PublicKey);

impl<'a> From<&'a PublicKey> for Verifier {
    fn from(public_key: &'a PublicKey) -> Self {
        Verifier(*public_key)
    }
}

impl signature::Verifier<Asn1Signature> for Verifier {
    fn verify(&self, msg: &[u8], signature: &Asn1Signature) -> Result<(), signature::Error> {
        UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, self.0.as_ref())
            .verify(msg, signature.as_ref())
            .map_err(|_| signature::Error::new())
    }
}

impl signature::Verifier<FixedSignature> for Verifier {
    fn verify(&self, msg: &[u8], signature: &FixedSignature) -> Result<(), signature::Error> {
        UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, self.0.as_ref())
            .verify(msg, signature.as_ref())
            .map_err(|_| signature::Error::new())
    }
}

#[cfg(test)]
mod tests {
    use super::{Signer, Verifier};
    use signatory::{
        ecdsa::nistp256::{test_vectors::SHA256_FIXED_SIZE_TEST_VECTORS, Asn1Signature},
        encoding::FromPkcs8,
        public_key::PublicKeyed,
        signature::{Signature as _, Signer as _, Verifier as _},
        test_vector::{TestVectorAlgorithm, ToPkcs8},
    };

    #[test]
    pub fn asn1_signature_roundtrip() {
        // TODO: DER test vectors
        let vector = &SHA256_FIXED_SIZE_TEST_VECTORS[0];
        let signer = Signer::from_pkcs8(&vector.to_pkcs8(TestVectorAlgorithm::NistP256)).unwrap();
        let signature: Asn1Signature = signer.sign(vector.msg);

        let verifier = Verifier::from(&signer.public_key().unwrap());
        assert!(verifier.verify(vector.msg, &signature).is_ok());
    }

    #[test]
    pub fn rejects_tweaked_asn1_signature() {
        let vector = &SHA256_FIXED_SIZE_TEST_VECTORS[0];
        let signer = Signer::from_pkcs8(&vector.to_pkcs8(TestVectorAlgorithm::NistP256)).unwrap();
        let signature: Asn1Signature = signer.sign(vector.msg);

        let mut tweaked_signature = signature.as_ref().to_vec();
        *tweaked_signature.iter_mut().last().unwrap() ^= 42;

        let verifier = Verifier::from(&signer.public_key().unwrap());
        let result = verifier.verify(
            vector.msg,
            &Asn1Signature::from_bytes(tweaked_signature.as_ref()).unwrap(),
        );

        assert!(
            result.is_err(),
            "expected bad signature to cause validation error!"
        );
    }
}
