//! Aes Encrypted communication and handshake process implementation

#![deny(missing_docs)]

use secp256k1::key::SecretKey;

pub use crate::handshake::handshake_struct::PublicKey;

/// Encrypted and decrypted codec implementation, and stream handle
pub mod codec;
/// Error type
pub mod error;
/// Exchange information during the handshake
mod exchange;
/// Implementation of the handshake process
pub mod handshake;
/// Encrypted stream
mod stream_cipher;
/// Supported algorithms
mod support;

/// Public key generated temporarily during the handshake
pub type EphemeralPublicKey = Vec<u8>;

/// Key pair of asymmetric encryption algorithm
#[derive(Clone, Debug)]
pub struct SecioKeyPair {
    inner: KeyPairInner,
}

impl SecioKeyPair {
    /// Generates a new random sec256k1 key pair.
    pub fn secp256k1_generated() -> SecioKeyPair {
        use rand::Rng;
        let mut random_slice = [0u8; secp256k1::constants::SECRET_KEY_SIZE];
        rand::thread_rng().fill(&mut random_slice[..]);
        let private = SecretKey::from_slice(&random_slice).expect("slice has the right size");
        SecioKeyPair {
            inner: KeyPairInner::Secp256k1 { private },
        }
    }

    /// Builds a `SecioKeyPair` from a raw secp256k1 32 bytes private key.
    pub fn secp256k1_raw_key<K>(key: K) -> SecioKeyPair
    where
        K: AsRef<[u8]>,
    {
        let private =
            secp256k1::key::SecretKey::from_slice(key.as_ref()).expect("slice has the right size");

        SecioKeyPair {
            inner: KeyPairInner::Secp256k1 { private },
        }
    }

    /// Returns the public key corresponding to this key pair.
    pub fn to_public_key(&self) -> PublicKey {
        match self.inner {
            KeyPairInner::Secp256k1 { ref private } => {
                let secp = secp256k1::Secp256k1::signing_only();
                let pubkey = secp256k1::key::PublicKey::from_secret_key(&secp, private);
                PublicKey::Secp256k1(pubkey.serialize().to_vec())
            }
        }
    }
}

#[derive(Clone, Debug)]
enum KeyPairInner {
    Secp256k1 { private: SecretKey },
}

/// Possible digest algorithms.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Digest {
    /// Sha256 digest
    Sha256,
    /// Sha512 digest
    Sha512,
}

impl Digest {
    /// Returns the size in bytes of a digest of this kind.
    #[inline]
    pub fn num_bytes(self) -> usize {
        match self {
            Digest::Sha256 => 256 / 8,
            Digest::Sha512 => 512 / 8,
        }
    }
}
