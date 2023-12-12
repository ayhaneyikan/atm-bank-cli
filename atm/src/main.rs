mod atm;
use crate::atm::ATM;
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

/// TCP address of the bank server
const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

/// ATM entrypoint
fn main() {
    let mut atm = ATM::new(BANK_SERVER_ADDR);

    // print initial prompt and flush buffer to terminal
    print!("{}", atm.get_prompt());
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

        // process given command
        if user_input.starts_with("begin-session") {
            atm.begin_session(&user_input);
            // atm.process_begin_session(&user_input, &mut stream); /* DO THIS FOR OTHERS */
        } else if user_input.starts_with("withdraw") {
            // atm.process_withdraw(&user_input, &mut stream);
        } else if user_input == "balance" {
            // atm.process_balance(&mut stream);
        } else if user_input == "end-session" {
            // atm.process_end_session(&mut stream);
        } else if user_input == "help" {
            println!("{}", atm.get_help());
        } else {
            println!("Invalid command\n");
        }

        // reprompt user
        print!("{}", atm.get_prompt());
        io::stdout().flush().unwrap();
        // clear user input buffer before next read
        user_input.clear();
    }
}
