use std::{env, io::{self, Write, Result}, net::TcpStream};

use rand::{rngs::OsRng, RngCore};
use x25519_dalek::{StaticSecret, PublicKey};

/* import atm structure */
mod atm;
use crate::atm::ATM;

// for encryption sources see: 
//  https://cryptography.rs/
//  https://kerkour.com/end-to-end-encryption-key-exchange-cryptography-rust
// based on these recommendations, I decided to utilize:
//  x25519-dalek        v1.2.0   for key exchange: https://crates.io/crates/x25519-dalek
//  blake2              v0.10.5  for KDF: https://crates.io/crates/blake2
//  XChaCha20-Poly1305  v0.10.1  for encryption: https://docs.rs/chacha20poly1305/latest/chacha20poly1305/

/* define program constants */
const BANK_SERVER_ADDR: &str = "127.0.0.1:32001";

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32usize;     // 32 byte key
const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24usize;   // 24 byte nonce


/*
 * ATM entry point
 */
fn main() {
    // retreive command line argument iterator and convert to vec
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     println!("\nThe ATM expects exactly one argument: a \"<>.atm\" file. Found {} arguments instead", args.len());
    //     return;
    // }


    /* generate key pair */

    // create and initialize nonce array
    // let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    // OsRng.fill_bytes(&mut nonce);  // randomize nonce values

    // let private_key: StaticSecret = StaticSecret::new(OsRng);
    // let public_key: PublicKey = PublicKey::from(&private_key);

    /* open connection to the bank */
    let mut stream: TcpStream = TcpStream::connect(BANK_SERVER_ADDR).expect(
        "Error trying to connect to the router"
    );

    // let listener: TcpListener = TcpListener::bind(ATM_ADDRESS).expect(
    //     "Error creating atm listener"
    // );
    
    // while let Err(_) = stream.write(public_key.as_bytes()) {}

    // listener.accept().expect("oops");
    

    // return;
    
    
    /* initialize atm struct */
    let mut atm: ATM = ATM::new();

    /* read in user input */
    let mut user_input: String = String::new();

    print!("{}", atm.get_prompt());  // initial prompt
    io::stdout().flush().unwrap();   // flush output buffer to terminal
    while io::stdin().read_line(&mut user_input).expect("Failed to read line from stdin") > 0 {
        /* remove newline from user input */
        user_input.pop();

        /* provide exit functionality */
        if user_input == "exit" {
            break;
        }


        stream.write(user_input.as_bytes()).unwrap();


        /* check for valid command and call appropriate helper function */
        if user_input.starts_with("begin-session") {
            atm.process_begin_session(&user_input);                             /* DO THIS FOR OTHERS */
        }
        else if user_input.starts_with("withdraw") {
            atm.process_withdraw(&user_input);
        }
        else if user_input == "balance" {
            atm.process_balance();
        }
        else if user_input == "end-session" {
            atm.process_end_session();
        }
        else if user_input == "help" {
            println!("  begin-session <user-name>");
            println!("  withdraw <amount>");
            println!("  balance");
            println!("  end-session");
            println!("  exit\n");
        }
        else {
            println!("Invalid command\n");
        }


        /* reprompt user */
        print!("{}", atm.get_prompt());
        io::stdout().flush().unwrap(); // flush prompt
        /* clear user input buffer before next read */
        user_input.clear();
    }
}
