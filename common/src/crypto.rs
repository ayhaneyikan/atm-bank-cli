use x25519_dalek::{EphemeralSecret, PublicKey};

// for encryption sources see:
//  https://cryptography.rs/
//  https://kerkour.com/end-to-end-encryption-key-exchange-cryptography-rust
// based on these recommendations, I decided to utilize:
//  x25519-dalek        v2       for key exchange: https://crates.io/crates/x25519-dalek
//  blake2              v0.10.5  for KDF: https://crates.io/crates/blake2
//  XChaCha20-Poly1305  v0.10.1  for encryption: https://docs.rs/chacha20poly1305/latest/chacha20poly1305/

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32; // 32 byte key
const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24; // 24 byte nonce

pub struct CryptoState {
    secret: EphemeralSecret,
    public: PublicKey,
}

impl CryptoState {
    /// Creates new CryptoState instance to manage cryptography
    pub fn new() -> Self {
        let secret = EphemeralSecret::random();
        Self {
            public: PublicKey::from(&secret),
            secret,
        }
    }
}
