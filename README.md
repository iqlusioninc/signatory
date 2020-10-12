# ![Signatory][logo]

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]

Pure Rust multi-provider digital signature library with support for elliptic
curve digital signature algorithms, namely ECDSA (described in [FIPS 186‑4])
and Ed25519 (described in [RFC 8032]).

Signatory provides a thread-safe and object-safe API and implements providers
for many popular Rust crates, including [secp256k1] and [sodiumoxide].

[Documentation][docs-link]

## About

Signatory exposes a thread-and-object-safe API for creating digital signatures
which allows several signature providers to be compiled-in and available with
specific providers selected at runtime.

## Minimum Supported Rust Version

All Signatory providers require Rust **1.44+**

## Provider Support

Signatory includes the following providers, which are each packaged into their
own respective crates:

| Provider Crate          | Backend Crate       | Type | Ed25519  | ECDSA/secp256k1 |
| ----------------------- | ------------------- | ---- | -------- | --------------- |
| [signatory‑secp256k1]   | [secp256k1]         | Soft | ⛔        | ✅              |
| [signatory‑sodiumoxide] | [sodiumoxide]       | Soft | ✅        | ⛔              |
| [signatory‑ledger-tm]   | [ledger-tendermint] | Hard | ✅        | ⛔              |

Above benchmarks performed using `cargo bench` on an Intel Xeon E3-1225 v5 @ 3.30GHz.

## License

**Signatory** is distributed under the terms of either the MIT license or the
Apache License (Version 2.0), at your option.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[//]: # (badges)

[logo]: https://storage.googleapis.com/iqlusion-production-web/github/signatory/signatory.svg
[crate-image]: https://img.shields.io/crates/v/signatory.svg
[crate-link]: https://crates.io/crates/signatory
[docs-image]: https://docs.rs/signatory/badge.svg
[docs-link]: https://docs.rs/signatory/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.44+-blue.svg
[build-image]: https://github.com/iqlusioninc/signatory/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/signatory/actions

[//]: # (general links)

[FIPS 186‑4]: https://csrc.nist.gov/publications/detail/fips/186/4/final
[RFC 8032]: https://tools.ietf.org/html/rfc8032
[secp256k1]: https://github.com/rust-bitcoin/rust-secp256k1/
[sodiumoxide]: https://github.com/dnaq/sodiumoxide
[ledger-tendermint]: https://crates.io/crates/ledger-tendermint
[signatory‑secp256k1]: https://crates.io/crates/signatory-secp256k1
[signatory‑sodiumoxide]: https://crates.io/crates/signatory-sodiumoxide
[signatory‑ledger-tm]: https://crates.io/crates/signatory-ledger-tm
