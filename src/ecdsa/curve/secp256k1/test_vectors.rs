//! ECDSA test vectors for secp256k1
//!
//! Sourced from the Python Cryptography library:
//!
//! https://github.com/pyca/cryptography/blob/master/vectors/cryptography_vectors/asymmetric/ECDSA/SECP256K1/SigGen.txt#L6314
//!
//! Generation procedure described here:
//!
//! https://cryptography.io/en/latest/development/custom-vectors/secp256k1/

#![cfg_attr(rustfmt, rustfmt_skip)]

use test_vector::TestVector;

/// secp256k1 raw ECDSA test vectors (from the Python Cryptography library)
// TODO: mark these as pub when we have a well-vetted set of test vectors
#[allow(dead_code)]
pub(crate) const RAW_TEST_VECTORS: &[TestVector] = &[
    // TODO: verify the conversions of these vectors from the upstream ones are correct and actually test against them
    TestVector {
        sk: b"\x13\x1c\xa4\xe5\x81\x12\x67\xfa\x90\xfc\x63\x1d\x62\x98\xc2\xd7\xa4\xec\xcc\xc4\x5c\xc6\x0d\x37\x8e\x06\x60\xb6\x1f\x82\xfe\x8d",
        pk: b"\xcf\x5a\xcf\x8e\xd3\xe0\xbb\xf7\x35\x30\x8c\xc4\x15\x60\x4b\xd3\x4a\xb8\xf7\xfc\x8b\x4a\x22\x74\x11\x17\xa7\xfb\xc7\x2a\x79\x49",
        msg: b"\x5c\x86\x8f\xed\xb8\x02\x69\x79\xeb\xd2\x6f\x1b\xa0\x7c\x27\xee\xdf\x4f\xf6\xd1\x04\x43\x50\x5a\x96\xec\xaf\x21\xba\x8c\x4f\x09\x37\xb3\xcd\x23\xff\xdc\x3d\xd4\x29\xd4\xcd\x19\x05\xfb\x8d\xbc\xce\xef\xf1\x35\x00\x20\xe1\x8b\x58\xd2\xba\x70\x88\x7b\xaa\x3a\x9b\x78\x3a\xd3\x0d\x3f\xbf\x21\x03\x31\xcd\xd7\xdf\x8d\x77\xde\xfa\x39\x8c\xda\xcd\xfc\x2e\x35\x9c\x7b\xa4\xca\xe4\x6b\xb7\x44\x01\xde\xb4\x17\xf8\xb9\x12\xa1\xaa\x96\x6a\xee\xba\x9c\x39\xc7\xdd\x22\x47\x9a\xe2\xb3\x07\x19\xdc\xa2\xf2\x20\x6c\x5e\xb4\xb7",
        sig: b"\xE5\x56\x43\x00\xC3\x60\xAC\x72\x90\x86\xE2\xCC\x80\x6E\x82\x8A\x84\x87\x7F\x1E\xB8\xE5\xD9\x74\xD8\x73\xE0\x65\x22\x49\x01\x55\x5F\xB8\x82\x15\x90\xA3\x3B\xAC\xC6\x1E\x39\x70\x1C\xF9\xB4\x6B\xD2\x5B\xF5\xF0\x59\x5B\xBE\x24\x65\x51\x41\x43\x8E\x7A\x10\x0B\xb2\xa9\xf6\xe8\x66\x0e\x5b\xc6\xeb\x78\xa8\x1b\x76\xc1\xeb\x9c\x56\xd8\xd6\x86\x0e\xaa\xbf\xe9\xcb\x58\xa3\x58\x75\x94\xcd\xfe"
    },
];
