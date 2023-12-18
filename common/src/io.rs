use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

use crate::{
    io::errors::ReceiveError,
    message::{
        constants::*,
        Plaintext, Response,
    },
};

pub const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

pub const AUTH_SUCCESS: u8 = 0;
pub const AUTH_FAILURE: u8 = 1;

/// Abstracts stream management away from bank and atm
pub struct StreamManager {
    stream: TcpStream,
}

impl StreamManager {
    //
    // constructors

    /// Consumes a stream and returns instance of a new stream manager
    pub fn from_stream(stream: TcpStream) -> Self {
        Self { stream }
    }
    /// Creates and returns instance of a new stream manager
    pub fn from_addr(addr: &str) -> Self {
        Self {
            stream: TcpStream::connect(addr)
                .expect("Error creating stream with provided address: {addr}"),
        }
    }

    //
    // low level send / receive helpers

    /// Writes a given plaintext to the stream and increments communication count
    pub fn send_plaintext(&mut self, mut plaintext: Plaintext) {
        // TODO encrypt
        self.stream.write(plaintext.get_bytes()).unwrap();
        plaintext.update_count();
    }

    /// Writes given buffer to the stream
    pub fn send_bytes(&mut self, message: &[u8]) {
        // TODO encrypt prior to send
        self.stream.write(message).unwrap();
    }

    pub fn receive(&mut self, comm_count: &mut u8) -> Result<Response, ReceiveError> {
        let mut buf = [0u8; MAX_PLAINTEXT_SIZE];
        if let Ok(0) = self.stream.read(&mut buf) {
            return Err(ReceiveError::EndOfStream);
        }
        // check for stale connection
        if buf[COMM_COUNTER_IDX] >= MAX_COMM_COUNTER {
            return Err(ReceiveError::StaleStream);
        }
        // check for external tampering
        if buf[COMM_COUNTER_IDX] != *comm_count {
            return Err(ReceiveError::InvalidCount);
        }
        *comm_count += 1;

        // TODO decrypt

        // construct Response
        Response::new(buf).map_err(|_| ReceiveError::InvalidMessage)
    }
}

/// Error types related IO
pub mod errors {
    use thiserror::Error;

    /// Error validating response received from stream
    #[derive(Debug, Error)]
    pub enum ReceiveError {
        /// Stream has been closed
        #[error("This stream has been closed")]
        EndOfStream,
        /// Maximum number of communications has been reached
        #[error("The maximum consecutive communications has been reached. Stream must be closed.")]
        StaleStream,
        /// Received message count did not match local count
        #[error("Message count did not match local count. An adversary may have dropped or replayed this message.")]
        InvalidCount,
        /// Received message type was unrecognized
        #[error("Received message type was unrecognized.")]
        InvalidMessage,
    }
}
