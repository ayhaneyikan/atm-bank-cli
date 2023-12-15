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

/// Index for communication counter byte
pub const COMM_COUNTER_IDX: usize = 0;
/// Maximum communication couter value
pub const MAX_COMM_COUNTER: u8 = u8::MAX - 2;

/// Index for message type byte
pub const MESSAGE_TYPE_IDX: usize = COMM_COUNTER_IDX + 1;

/// Index for start of plaintext body
pub const MESSAGE_START_IDX: usize = MESSAGE_TYPE_IDX + 1;
/// Maximum length of message body
pub const MESSAGE_BODY_SIZE: usize = MAX_USERNAME_SIZE + PIN_SIZE;

/// Index for start of username within plaintext
pub const USERNAME_START_IDX: usize = MESSAGE_START_IDX;
/// Maximum length of username
pub const MAX_USERNAME_SIZE: usize = 20;
/// Index for end of username within plaintext
pub const USERNAME_END_IDX: usize = USERNAME_START_IDX + MAX_USERNAME_SIZE - 1;

/// Index for start of PIN within plaintext
pub const PIN_START_IDX: usize = USERNAME_START_IDX + MAX_USERNAME_SIZE;
/// Length of PIN
pub const PIN_SIZE: usize = 4;
/// Index for end of PIN within plaintext
pub const PIN_END_IDX: usize = PIN_START_IDX + PIN_SIZE - 1;

/// Length of the entire plaintext
pub const MAX_PLAINTEXT_SIZE: usize = 1 + 1 + MESSAGE_BODY_SIZE;

// End message constants
//

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
