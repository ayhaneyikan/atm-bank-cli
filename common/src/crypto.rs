use x25519_dalek::{EphemeralSecret, PublicKey};

// for encryption sources see:
//  https://cryptography.rs/
//  https://kerkour.com/end-to-end-encryption-key-exchange-cryptography-rust
// based on these recommendations, I decided to utilize:
//  x25519-dalek        v2       for key exchange: https://crates.io/crates/x25519-dalek
//  blake2              v0.10.5  for KDF: https://crates.io/crates/blake2
//  XChaCha20-Poly1305  v0.10.1  for encryption: https://docs.rs/chacha20poly1305/latest/chacha20poly1305/

//
// Message constants

// Example message
// byte # | purpose
// 0        communicating atm request type
// 1-20     username up to 20 characters
// 21-24    pin exactly 4 characters

/// Maximum length of a username
pub const MAX_USERNAME_SIZE: usize = 20;
/// Length of a PIN
pub const PIN_SIZE: usize = 4;
/// Maximum length of an entire plaintext
pub const MAX_PLAINTEXT_SIZE: usize = 1 + MAX_USERNAME_SIZE + PIN_SIZE;
/// Starting index of username within plaintext
pub const USERNAME_START_IDX: usize = 1;
/// Starting index of pin within plaintext
pub const PIN_START_IDX: usize = 21;
/// End index of username within plaintext. Meant to be used INCLUSIVELY
pub const USERNAME_END_IDX: usize = USERNAME_START_IDX + MAX_USERNAME_SIZE - 1;
/// End index of pin within plaintext. Meant to be used INCLUSIVELY
pub const PIN_END_IDX: usize = PIN_START_IDX + PIN_SIZE - 1;

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32usize; // 32 byte key
const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24usize; // 24 byte nonce

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
