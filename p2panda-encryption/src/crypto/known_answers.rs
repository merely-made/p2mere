// SPDX-License-Identifier: MIT OR Apache-2.0

//! Known-answer tests for the primitives this crate takes from RustCrypto and rand_chacha.
//!
//! The HKDF and SHA-2 vectors are the published ones (RFC 5869 case 1, FIPS 180 "abc"). The
//! AEAD and RNG outputs were recorded with the previous library releases (chacha20poly1305 0.10,
//! rand_chacha 0.9), so a dependency bump that changed a byte on the wire fails here.
use super::Rng;
use super::aead::aead_encrypt;
use super::hkdf::hkdf;
use super::sha2::{sha2_256, sha2_512};
use super::xchacha20::x_aead_encrypt;

fn hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[test]
fn sha2_abc() {
    assert_eq!(
        hex(&sha2_256(&[b"a", b"bc"])),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        hex(&sha2_512(&[b"abc"])),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
         2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
}

#[test]
fn hkdf_rfc5869_case_1() {
    let ikm = [0x0b; 22];
    let salt: Vec<u8> = (0x00..=0x0c).collect();
    let info: Vec<u8> = (0xf0..=0xf9).collect();
    let okm: [u8; 42] = hkdf(&salt, &ikm, Some(&info)).unwrap();
    assert_eq!(
        hex(&okm),
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
    );
}

#[test]
fn rng_stream() {
    let rng = Rng::from_seed([7; 32]);
    assert_eq!(hex(&rng.random_vec(48).unwrap()), RNG_STREAM);
}

#[test]
fn aead_ciphertexts() {
    let key = [0x42; 32];
    let ct = aead_encrypt(&key, b"Hello, Panda!", [0x24; 12], Some(b"aad")).unwrap();
    assert_eq!(hex(&ct), AEAD_CT);
    let ct = x_aead_encrypt(&key, b"Hello, Panda!", [0x24; 24], Some(b"aad")).unwrap();
    assert_eq!(hex(&ct), XAEAD_CT);
}

const RNG_STREAM: &str = "f400927857aaf64114f561baacb379708c79a1dc1476ab573216a4020764bde545c143dbb9609c22ab855d60925a9976";
const AEAD_CT: &str = "ac62e96284f278f8e405eceac5fb5a727e7a5e06a39075c5abc33690ec";
const XAEAD_CT: &str = "ed3bbb0901086cf6c3415b5b2f587261d861a735e08082253777a1cf70";
