use std::{
    io::{self, Error, Read, Write},
    net::TcpStream,
    str,
};

use crate::crypto::{COMM_COUNTER_IDX, MAX_PLAINTEXT_SIZE, MESSAGE_TYPE_IDX};

pub const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

pub const AUTH_SUCCESS: u8 = 0;
pub const AUTH_FAILURE: u8 = 1;

#[repr(u8)]
pub enum RequestType {
    AuthUser,
    Balance,
    Withdraw,
    Deposit,
    End,
}

impl TryFrom<u8> for RequestType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AuthUser),
            1 => Ok(Self::Balance),
            2 => Ok(Self::Withdraw),
            3 => Ok(Self::Deposit),
            4 => Ok(Self::End),
            _ => Err(()),
        }
    }
}

pub fn create_plaintext(comm_count: u8, request: RequestType) -> [u8; MAX_PLAINTEXT_SIZE] {
    let mut plaintext = [0u8; MAX_PLAINTEXT_SIZE];
    plaintext[COMM_COUNTER_IDX] = comm_count;
    plaintext[MESSAGE_TYPE_IDX] = request as u8;
    plaintext
}
pub fn insert_bytes_into_plaintext(plaintext: &mut [u8], bytes: &[u8], offset: usize) {
    for i in 0..bytes.len() {
        plaintext[i + offset] = bytes[i];
    }
}

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

    /// Writes given buffer to the stream
    pub fn send(&mut self, message: &[u8]) {
        // TODO encrypt prior to send
        self.stream.write(message).unwrap();
    }
    /// Reads from stream and stores in provided output buffer
    pub fn receive(&mut self, output_buf: &mut [u8]) -> Result<usize, ()> {
        match self.stream.read(output_buf) {
            Ok(n) => match n {
                0 => Err(()),
                _ => Ok(n),
            },
            Err(_) => Err(()),
        }
        // TODO decrypt after read
    }
    /// Reads from stream and returns the message as a `String`
    pub fn receive_as_str(&mut self) -> String {
        let mut response = [0u8; MAX_PLAINTEXT_SIZE];
        self.stream.read(&mut response).unwrap();
        // TODO decrypt and separate out message counter
        // convert to string
        str::from_utf8(&response)
            .unwrap()
            .trim_end_matches(|c| c == '\0')
            .to_string()
    }
}
