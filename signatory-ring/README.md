# DEPRECATED: signatory-ring

The new `ring-compat` crate implements the [`signature::Signer`] and
[`signature::Verifier`] traits used by Signatory, for ECDSA/p256, ECDSA/p384,
and Ed25519 signatures:

https://github.com/RustCrypto/ring-compat

Please migrate to `ring-compat` if you were formerly using `signatory-ring`.

[`signature::Signer`]: https://docs.rs/signature/latest/signature/trait.Signer.html
[`signature::Verifier`]: https://docs.rs/signature/latest/signature/trait.Verifier.html
