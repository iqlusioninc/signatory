# DEPRECATED: signatory-dalek

As of `ed25519-dalek` v1.0.0-pre.4, the [`signature::Signer`] and
[`signature::Verifier`] traits used by Signatory are now
[natively implemented by the `ed25519-dalek` crate itself](https://github.com/dalek-cryptography/ed25519-dalek/pull/124).

This means a `signatory-dalek` crate is no longer necessary, because you can
just use `ed25519-dalek` directly now:

<https://github.com/dalek-cryptography/ed25519-dalek>

[`signature::Signer`]: https://docs.rs/signature/latest/signature/trait.Signer.html
[`signature::Verifier`]: https://docs.rs/signature/latest/signature/trait.Verifier.html
