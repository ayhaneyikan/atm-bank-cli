mod atm;
use crate::atm::ATM;
use common::io::BANK_SERVER_ADDR;
use std::io::{self, Write};

// for encryption sources see:
//  https://cryptography.rs/
//  https://kerkour.com/end-to-end-encryption-key-exchange-cryptography-rust
// based on these recommendations, I decided to utilize:
//  x25519-dalek        v2       for key exchange: https://crates.io/crates/x25519-dalek
//  blake2              v0.10.5  for KDF: https://crates.io/crates/blake2
//  XChaCha20-Poly1305  v0.10.1  for encryption: https://docs.rs/chacha20poly1305/latest/chacha20poly1305/

//
// program constants

/// ATM entrypoint
fn main() {
    let mut atm = ATM::new(BANK_SERVER_ADDR);

    // print initial prompt and flush buffer to terminal
    println!("\nAvailable commands:\n{}", atm.get_help_display());
    print!("\n{}", atm.get_prompt());
    io::stdout().flush().unwrap();

    // user input buffer
    let mut user_input = String::new();

    // iteratively read user input
    while io::stdin().read_line(&mut user_input).unwrap() > 0 {
        // remove newline from end of user input
        user_input.pop();

        // provide exit functionality
        if user_input == "exit" {
            break;
        }

        atm.process_input(&user_input.trim());

        // reprompt user
        print!("\n{}", atm.get_prompt());
        io::stdout().flush().unwrap();
        // clear user input buffer before next read
        user_input.clear();
    }
}
