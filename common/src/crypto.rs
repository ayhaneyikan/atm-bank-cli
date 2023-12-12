use x25519_dalek::{EphemeralSecret, PublicKey};

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
