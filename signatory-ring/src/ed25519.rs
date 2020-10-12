//! Ed25519 signer and verifier implementation for *ring*

pub use signatory::ed25519::{PublicKey, Seed, Signature};

use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey};
use signatory::signature::{self, Signature as _};

#[cfg(feature = "std")]
use ring::rand::SystemRandom;
#[cfg(feature = "std")]
use signatory::encoding::{
    self,
    pkcs8::{self, FromPkcs8, GeneratePkcs8},
};

/// Ed25519 signature provider for *ring*
pub struct Signer(Ed25519KeyPair);

impl From<&Seed> for Signer {
    /// Create a new Ed25519Signer from an unexpanded seed value
    fn from(seed: &Seed) -> Self {
        let keypair = Ed25519KeyPair::from_seed_unchecked(seed.as_secret_slice()).unwrap();
        Signer(keypair)
    }
}

#[cfg(feature = "std")]
impl FromPkcs8 for Signer {
    /// Create a new Ed25519Signer from a PKCS#8 encoded private key
    fn from_pkcs8<K: AsRef<[u8]>>(secret_key: K) -> Result<Self, encoding::Error> {
        let keypair = Ed25519KeyPair::from_pkcs8(secret_key.as_ref())
            .map_err(|_| encoding::error::ErrorKind::Decode)?;

        Ok(Signer(keypair))
    }
}

#[cfg(feature = "std")]
impl GeneratePkcs8 for Signer {
    /// Randomly generate an Ed25519 **PKCS#8** keypair
    fn generate_pkcs8() -> Result<pkcs8::SecretKey, encoding::Error> {
        let keypair = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap();
        pkcs8::SecretKey::from_bytes(keypair.as_ref())
    }
}

impl From<&Signer> for PublicKey {
    fn from(signer: &Signer) -> PublicKey {
        PublicKey::from_bytes(signer.0.public_key()).unwrap()
    }
}

impl signature::Signer<Signature> for Signer {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, signature::Error> {
        Ok(Signature::from_bytes(self.0.sign(msg).as_ref()).unwrap())
    }
}

/// Ed25519 verifier for *ring*
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verifier(PublicKey);

impl<'a> From<&'a PublicKey> for Verifier {
    fn from(public_key: &'a PublicKey) -> Self {
        Verifier(*public_key)
    }
}

impl signature::Verifier<Signature> for Verifier {
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), signature::Error> {
        UnparsedPublicKey::new(&ring::signature::ED25519, self.0.as_bytes())
            .verify(msg, signature.as_ref())
            .map_err(|_| signature::Error::new())
    }
}

#[cfg(test)]
mod tests {
    use super::{Signer, Verifier};
    signatory::ed25519_tests!(Signer, Verifier);
}
