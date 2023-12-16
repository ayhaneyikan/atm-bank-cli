/// Structural constants defining message size and makeup
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

use self::errors::{MessageTypeError, ResponseError};
use crate::{
    io::{AUTH_FAILURE, AUTH_SUCCESS},
    message::constants::*,
};
use std::str;

/// Enum representing possible message types sent and received
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    AuthUser,
    Balance,
    Withdraw,
    Deposit,
    End,
    AuthResult,
}

impl TryFrom<u8> for MessageType {
    type Error = MessageTypeError;
    /// Conversion from u8 to MessageType
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AuthUser),
            1 => Ok(Self::Balance),
            2 => Ok(Self::Withdraw),
            3 => Ok(Self::Deposit),
            4 => Ok(Self::End),
            5 => Ok(Self::AuthResult),
            _ => Err(MessageTypeError::InvalidType(value)),
        }
    }
}

/// Provides interfaces for creating and managing a message plaintext
pub struct Plaintext<'a> {
    contents: [u8; MAX_PLAINTEXT_SIZE],
    comm_count: &'a mut u8,
}

impl<'a> Plaintext<'a> {
    /// Create a new instance of an ATM plaintext
    pub fn new(comm_count: &'a mut u8, msg_type: MessageType) -> Self {
        let mut ptext = [0u8; MAX_PLAINTEXT_SIZE];
        ptext[COMM_COUNTER_IDX] = *comm_count;
        ptext[MESSAGE_TYPE_IDX] = msg_type as u8;
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

    /// Converts this plaintext into an auth result message
    pub fn set_auth_result(&mut self, result: bool) {
        self.contents[MESSAGE_START_IDX] = match result {
            true => AUTH_SUCCESS,
            false => AUTH_FAILURE,
        };
    }
}

/// Provides a friendlier interface with a received plaintext
pub struct Response {
    msg_type: MessageType,
    contents: [u8; MAX_PLAINTEXT_SIZE],
}

impl Response {
    /// Creates new instance of a message Response.
    /// May fail to create if provided message has unrecognized type
    pub fn new(response_buf: [u8; MAX_PLAINTEXT_SIZE]) -> Result<Self, MessageTypeError> {
        let msg_type = MessageType::try_from(response_buf[MESSAGE_TYPE_IDX])?;
        Ok(Self {
            msg_type,
            contents: response_buf,
        })
    }

    //
    // getters

    /// Returns this response's message type
    pub fn get_type(&self) -> MessageType {
        self.msg_type
    }

    /// Returns username string or error
    pub fn get_user(&self) -> Result<String, ResponseError> {
        if !matches!(self.msg_type, MessageType::AuthUser) {
            return Err(ResponseError::DeconstructError {
                request: MessageType::AuthUser,
                actual: self.msg_type,
            });
        }
        Ok(
            str::from_utf8(&self.contents[USERNAME_START_IDX..=USERNAME_END_IDX])
                .map_err(|_| ResponseError::InvalidBytesForString)?
                .trim_end_matches(|c| c == '\0')
                .to_string(),
        )
    }

    /// Returns pin u16 or error
    pub fn get_pin(&self) -> Result<u16, ResponseError> {
        if !matches!(self.msg_type, MessageType::AuthUser) {
            return Err(ResponseError::DeconstructError {
                request: MessageType::AuthUser,
                actual: self.msg_type,
            });
        }
        str::from_utf8(&self.contents[PIN_START_IDX..=PIN_END_IDX])
            .map_err(|_| ResponseError::InvalidBytesForString)?
            .trim_end_matches(|c| c == '\0')
            .parse()
            .map_err(|_| ResponseError::InvalidBytesForPIN)
    }

    /// Returns result of authentication or error
    pub fn get_auth_result(&self) -> Result<bool, ResponseError> {
        if !matches!(self.msg_type, MessageType::AuthResult) {
            return Err(ResponseError::DeconstructError {
                request: MessageType::AuthResult,
                actual: self.msg_type,
            });
        }
        Ok(self.contents[MESSAGE_START_IDX] == AUTH_SUCCESS)
    }
}

/// Error types related to messages
pub mod errors {
    use thiserror::Error;

    use super::MessageType;

    /// Errors trying to create a MessageType
    #[derive(Debug, Error)]
    pub enum MessageTypeError {
        #[error("MessageType cannot be created from u8 value: `{0}`")]
        InvalidType(u8),
    }

    #[derive(Debug, Error)]
    pub enum ResponseError {
        /// Invalid message type in received message
        #[error("Cannot create Response with invalid message type.")]
        InvalidMessageType(#[from] crate::message::MessageTypeError),
        /// Unable to retrieve the requested field because it does not exist in the received message type
        #[error("Cannot retreive requested {request:?} field from a message of type {actual:?}.")]
        DeconstructError {
            request: MessageType,
            actual: MessageType,
        },
        /// Failed conversion from received bytes into string
        #[error("Cannot convert message body into a valid username string.")]
        InvalidBytesForString,
        /// Failed u16 PIN conversion from received byte string
        #[error("Cannot covert message body into a valid PIN.")]
        InvalidBytesForPIN,
    }
}
