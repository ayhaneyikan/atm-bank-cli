use common::crypto::{
    CryptoState, MAX_PLAINTEXT_SIZE, MAX_USERNAME_SIZE, PIN_SIZE, PIN_START_IDX, USERNAME_START_IDX,
};
use common::io::RequestType;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

type Username = String;

enum ATMState {
    BASE,
    LOGGED(Username),
}

/// Maintains ATM state and facilitates communications with the bank
pub struct ATM {
    state: ATMState,
    stream: TcpStream,
    crypto: CryptoState,
}

impl ATM {
    /// Create new ATM instance
    pub fn new(bank_address: &str) -> Self {
        Self {
            state: ATMState::BASE,
            stream: TcpStream::connect(bank_address)
                .expect("Error trying to connect to the bank server"),
            crypto: CryptoState::new(),
        }
    }

    //
    // prompt retreival helpers and input processing

    /// Returns CLI prompt based on current ATM state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            ATMState::BASE => "ATM: ".to_string(),
            ATMState::LOGGED(user) => format!("ATM ({}): ", user),
        }
    }
    /// Returns CLI help list
    pub fn get_help_display(&self) -> String {
        match self.state {
            ATMState::BASE => "  begin-session <user-name> <PIN>\n".to_string() + "  help\n" + "  exit",
            ATMState::LOGGED(_) => {
                "  withdraw <amount>\n".to_string()
                    + "  balance\n"
                    + "  end-session\n"
                    + "  help\n"
                    + "  exit"
            }
        }
    }
    /// Processes user input based on content.
    /// Limits accessibility of certain commands based on state.
    pub fn process_input(&mut self, input: &str) {
        match self.state {
            ATMState::BASE => {
                if input.starts_with("begin-session") {
                    if input.trim() == "begin-session" {
                        println!("Usage: begin-session <user-name> <PIN>");
                        return;
                    }
                    self.begin_session(input);
                } else if input == "help" {
                    println!("{}", self.get_help_display());
                } else {
                    println!("Invalid command. Use `help` to see options.");
                }
            }
            ATMState::LOGGED(_) => {
                if input.starts_with("withdraw") {
                    // atm.process_withdraw(&user_input, &mut stream);
                } else if input == "balance" {
                    // atm.process_balance(&mut stream);
                } else if input == "end-session" {
                    // atm.process_end_session(&mut stream);
                } else if input == "help" {
                    println!("{}", self.get_help_display());
                } else {
                    println!("Invalid command. Use `help` to see options.");
                }
            }
        }
    }

    //
    // helpers for managing atm logic

    /// Returns bool indicating whether there is an active user
    fn is_active_user(&self) -> bool {
        match self.state {
            ATMState::BASE => false,
            ATMState::LOGGED(_) => true,
        }
    }

    //
    // methods for processing commands

    /// Handles user request to begin authenticated session with the bank.
    /// This method cannot be reached if a user is already logged in.
    fn begin_session(&mut self, user_input: &str) {
        // create static regular expression (cached to avoid re-creations)
        lazy_static! {
            static ref BS_RE: Regex = Regex::new("^begin-session ([a-zA-Z0-9]+) ([0-9]{4})$")
                .expect("Error while compiling begin-session regular expression");
        }

        // early exit if invalid command
        if !BS_RE.is_match(user_input) {
            println!("Usage: begin-session <user-name> <PIN>\n");
            return;
        }

        //
        // extract fields from input

        // extract username and PIN
        let caps = BS_RE.captures(user_input).unwrap();
        let username = caps.get(1).unwrap().as_str();
        let pin = caps.get(2).unwrap().as_str();
        // PIN length is fixed
        // validate username length
        if username.len() > MAX_USERNAME_SIZE {
            println!(
                "Error: username must be {} characters or less\n",
                MAX_USERNAME_SIZE
            );
            return;
        }

        //
        // construct and send authentication request

        let mut plaintext = [0u8; MAX_PLAINTEXT_SIZE];
        // set message type
        plaintext[0] = RequestType::AuthUser as u8;
        // add username
        for i in 0..username.len() {
            plaintext[i + USERNAME_START_IDX] = username.as_bytes()[i];
        }
        // add PIN
        for i in 0..pin.len() {
            plaintext[i + PIN_START_IDX] = pin.as_bytes()[i];
        }

        // TODO encrypt prior to send
        // send plaintext to bank
        self.stream.write(&plaintext).unwrap();

        // receive response
        let mut resp = [0u8; MAX_PLAINTEXT_SIZE];
        self.stream.read(&mut resp).unwrap();
        // TODO decrypt after receive

        // TODO include trim as part of decryption process
        let resp = str::from_utf8(&resp)
            .unwrap()
            .trim_end_matches(|c| c == '\0');

        if resp != "success" {
            println!("Authentication failed");
            return;
        }

        // update login state
        self.state = ATMState::LOGGED(username.to_string());
        println!("Authorization successful");
        println!("Available commands:\n{}", self.get_help_display());
    }

    //     /* attempt to begin a user session */
    //     pub fn process_begin_session(&mut self, user_input: &String, stream: &mut TcpStream) {
    //         /* verify that no user is already logged in */
    //         if self.is_active_user() {
    //             println!("A user is already logged in\n");
    //             return;
    //         }

    //         /* create static regular expression (creates it at most once) */
    //         lazy_static! {
    //             static ref RE: Regex = Regex::new("^begin-session ([a-zA-Z]+)$")
    //                 .expect("Error while compiling begin-session regular expression");
    //         }

    //         /* ensure input matches */
    //         if !RE.is_match(user_input) {
    //             println!("Usage: begin-session <user-name>\n");
    //             return;
    //         }
    //         /* valid match */
    //         /* extract username from matched string */
    //         let caps: Captures = RE.captures(user_input).unwrap();
    //         let username: &str = caps.get(1).unwrap().as_str();
    //         /* check if username is within max size */
    //         if username.len() > MAX_USERNAME_SIZE {
    //             println!(
    //                 "Error: username must be {} characters or less\n",
    //                 MAX_USERNAME_SIZE
    //             );
    //             return;
    //         }

    //         /* check if they exist communiate with bank */
    //         /* construct plaintext */
    //         let mut plaintext = [0u8; MAX_PLAINTEXT_SIZE];

    //         for i in 0..username.len() {
    //             plaintext[i + 1] = username.as_bytes()[i];
    //         }

    //         /* send plaintext to bank */
    //         while let Err(_) = ATM::send_message(&plaintext, stream) {}

    //         /* listen for response */
    //         let mut resp = [0u8; MAX_PLAINTEXT_SIZE];
    //         while let Err(_) = ATM::recv_message(&mut resp, stream) {}

    //         println!("{:?}", str::from_utf8(&resp).unwrap());

    //         if resp != "success".as_bytes() {
    //             println!("No such user\n");
    //             return;
    //         }

    //         /* prompt for PIN */
    //         print!("  PIN: ");
    //         io::stdout().flush().unwrap(); // flush prompt
    //                                        /* capture PIN */
    //         let mut pin: String = String::new();
    //         io::stdin()
    //             .read_line(&mut pin)
    //             .expect("Failed to read PIN from stdin");
    //         pin.pop(); // remove newline
    //                    /* check if provided PIN matches */
    //         let failed_authorization: String = String::from("Not authorized\n");
    //         lazy_static! {
    //             static ref PIN_RE: Regex = Regex::new("^[0-9]{4}$").unwrap();
    //         }
    //         if !PIN_RE.is_match(&pin) {
    //             println!("{}", failed_authorization);
    //             return;
    //         }

    //         /* validate PIN with bank */
    //  /* TEMP */
    //         if pin != "1188" {
    //             println!("{}", failed_authorization);
    //             return;
    //         }

    //         /* update login state */
    //         self.login_user(username);
    //         println!("Authorized\n");
    //     }

    //     pub fn process_withdraw(&mut self, user_input: &String, stream: &mut TcpStream) {
    //         /* verify there is a user logged in */
    //         if !self.is_active_user() {
    //             println!("No user logged in\n");
    //             return;
    //         }

    //         /* create static regular expression (creates it at most once) */
    //         lazy_static! {
    //             static ref RE: Regex = Regex::new("^withdraw ([0-9]+)$")
    //                 .expect("Error while compiling withdraw regular expression");
    //         }

    //         /* ensure input matches */
    //         if !RE.is_match(user_input) {
    //             println!("Usage: withdraw <amt>\n");
    //             return;
    //         }

    //         /* extract withdrawal amount from input */
    //         let caps: Captures = RE.captures(user_input).unwrap();
    //         let amount_string: &str = caps.get(1).unwrap().as_str();

    //         /* check if valid u64 */
    //         let amount: u64;
    //         if let Ok(extracted_value) = amount_string.parse::<u64>() {
    //             amount = extracted_value;
    //         } else {
    //             println!("Error: your requested withdrawal amount is too large for our wee little bank to handle\n");
    //             return;
    //         }

    //         /* attempt withdrawal from bank */
    //  /* TEMP */
    //         if amount > 1_000_000u64 {
    //             println!("Insufficient funds\n");
    //             return;
    //         }
    //         println!("${} dispensed\n", amount);
    //     }

    //     pub fn process_balance(&mut self, stream: &mut TcpStream) {
    //         /* verify there is a user logged in */
    //         if !self.is_active_user() {
    //             println!("No user logged in\n");
    //             return;
    //         }

    //         /* no regular expression needed */
    //         /* retreive balance from the bank */
    //     }

    //     pub fn process_end_session(&mut self, stream: &mut TcpStream) {
    //         /* check if there's a user logged in */
    //         if !self.is_active_user() {
    //             println!("No user logged in\n");
    //             return;
    //         }

    //         println!(
    //             "{} was logged out\n",
    //             self.active_user
    //                 .as_ref()
    //                 .expect("Error: active user should never be none here")
    //         );
    //         self.logout_user();
    //     }
}
