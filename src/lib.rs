//! Signatory

//#![no_std]
#![doc(html_root_url = "https://docs.rs/signatory/0.23.0-pre")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[allow(unused_extern_crates)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod keystore;

#[cfg(feature = "ecdsa")]
pub mod ecdsa;

mod error;
mod keyring;

pub use self::{
    error::{Error, Result},
    keyring::KeyRing,
};
pub use signature;
