//! Traits for public keys

use core::fmt::Debug;

/// Common trait for all public keys
pub trait PublicKey: AsRef<[u8]> + Debug + Sized + Eq + Ord {}
