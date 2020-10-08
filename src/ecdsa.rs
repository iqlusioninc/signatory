//! The Elliptic Curve Digital Signature Algorithm (ECDSA) as specified in
//! FIPS 186-4 (Digital Signature Standard)

#[cfg(feature = "p256")]
pub mod nistp256;

#[cfg(feature = "p384")]
pub mod nistp384;

#[cfg(feature = "k256")]
pub mod secp256k1;

// Re-export key types from the `elliptic_curve` crate
pub use ::ecdsa::elliptic_curve::{
    self,
    sec1::{EncodedPoint as PublicKey, UncompressedPointSize, UntaggedPointSize},
    weierstrass::Curve,
    SecretKey,
};

// Use signature types from the `ecdsa` crate
pub use ::ecdsa::{asn1::Signature as Asn1Signature, generic_array, Signature as FixedSignature};

use core::ops::Add;
use generic_array::{typenum::U1, ArrayLength};

impl<C> crate::public_key::PublicKey for PublicKey<C>
where
    C: Curve,
    UntaggedPointSize<C>: Add<U1> + ArrayLength<u8>,
    UncompressedPointSize<C>: ArrayLength<u8>,
{
}

/// ECDSA test vector
#[cfg(feature = "test-vectors")]
pub struct TestVector {
    /// Secret key
    pub sk: &'static [u8],

    /// Public key
    pub pk: &'static [u8],

    /// Nonce (i.e. ECDSA `k` value)
    pub nonce: Option<&'static [u8]>,

    /// Message
    pub msg: &'static [u8],

    /// Signature
    pub sig: &'static [u8],
}
