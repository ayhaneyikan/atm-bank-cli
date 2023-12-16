pub mod constants {
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
}

use crate::{io::RequestType, message::constants::*};

/// Provides interfaces for creating and managing a message plaintext
pub struct Plaintext<'a> {
    contents: [u8; MAX_PLAINTEXT_SIZE],
    comm_count: &'a mut u8,
}

impl<'a> Plaintext<'a> {
    /// Create a new instance of an ATM plaintext
    pub fn new(comm_count: &'a mut u8, request: RequestType) -> Self {
        let mut ptext = [0u8; MAX_PLAINTEXT_SIZE];
        ptext[COMM_COUNTER_IDX] = *comm_count;
        ptext[MESSAGE_TYPE_IDX] = request as u8;
        Self {
            contents: ptext,
            comm_count,
        }
    }
    /// Returns reference to the message
    pub fn get_bytes(&self) -> &[u8] {
        &self.contents
    }
    /// Updates comm count reference after send
    pub fn update_count(&mut self) {
        *self.comm_count += 1;
    }

    //
    // helpers

    /// Resets the message body contents to avoid possible overlaps caused by
    /// user error: calling multiple inits
    fn reset_body(&mut self) {
        for i in MESSAGE_START_IDX..MAX_PLAINTEXT_SIZE {
            self.contents[i] = 0;
        }
    }
    /// Generically inserts bytes into the plaintext
    fn generic_insert(&mut self, bytes: &[u8], offset: usize) {
        self.contents[offset..(bytes.len() + offset)].copy_from_slice(bytes);
    }

    //
    // message type setters

    /// Converts this plaintext into an auth user message
    pub fn set_auth_user(&mut self, username: &str, pin: &str) {
        self.reset_body();
        self.generic_insert(username.as_bytes(), USERNAME_START_IDX);
        self.generic_insert(pin.as_bytes(), PIN_START_IDX);
    }
}

/// Provides a friendlier interface with a received plaintext
pub struct Response {
    count: u8,
    message_type: RequestType,
    contents: [u8; MESSAGE_BODY_SIZE],
}
