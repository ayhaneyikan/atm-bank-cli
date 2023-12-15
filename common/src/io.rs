use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str,
};

use crate::crypto::MAX_PLAINTEXT_SIZE;

pub const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

#[repr(u8)]
pub enum RequestType {
    AuthUser,
}

impl TryFrom<u8> for RequestType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AuthUser),
            _ => Err(()),
        }
    }
}

pub struct StreamManager {
    stream: TcpStream,
}

impl StreamManager {
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
    // Low level send / receive helpers

    /// Writes given buffer to the stream
    pub fn send(&mut self, message: &[u8]) {
        self.stream.write(message).unwrap();
    }
    /// Reads from stream and stores in provided output buffer
    pub fn receive(&mut self, output_buf: &mut [u8]) {
        self.stream.read(output_buf).unwrap();
    }
    /// Reads from stream and returns the message as a `String`
    pub fn receive_as_str(&mut self) -> String {
        let mut response = [0u8; MAX_PLAINTEXT_SIZE];
        self.stream.read(&mut response).unwrap();
        // TODO separate out message counter
        // convert to string
        str::from_utf8(&response)
            .unwrap()
            .trim_end_matches(|c| c == '\0')
            .to_string()
    }
}
