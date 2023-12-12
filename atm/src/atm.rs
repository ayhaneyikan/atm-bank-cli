use common::crypto::CryptoState;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

// ATM constants
const MAX_USERNAME_SIZE: usize = 250;
const MAX_PLAINTEXT_SIZE: usize = MAX_USERNAME_SIZE + 1;

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

    /// Returns CLI prompt based on current ATM state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            ATMState::BASE => "ATM: ".to_string(),
            ATMState::LOGGED(user) => format!("ATM ({}): ", user),
        }
    }
    /// Returns CLI help list
    pub fn get_help(&self) -> String {
        match self.state {
            ATMState::BASE => "  begin-session <user-name>\n".to_string() + "  exit\n",
            ATMState::LOGGED(_) => {
                "  withdraw <amount>\n".to_string() + "  balance\n" + "  end-session\n" + "  exit\n"
            }
        }
    }

    /// Updates ATM state after a user requests a login
    fn login_user(&mut self, username: &str) {
        if matches!(self.state, ATMState::BASE) {
            // TODO: make login request to bank
            self.state = ATMState::LOGGED(username.to_string());
        }
        // TODO: revisit this edge: case login after logged in
    }

    /// Updates ATM state after a user logs out
    fn logout_user(&mut self) {
        if matches!(self.state, ATMState::LOGGED(_)) {
            // TODO: any communication necessary with bank?
            self.state = ATMState::BASE;
        }
    }

    /// Returns bool indicating whether there is an active user
    fn is_active_user(&self) -> bool {
        match self.state {
            ATMState::BASE => false,
            ATMState::LOGGED(_) => true,
        }
    }

    /// sends message by writing into the provided TcpStream
    ///
    /// returns Result indicating success status of the write
    fn send_message(&mut self, plaintext: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.write(plaintext)
    }
    /// receives message by reading from the provided TcpStream
    ///
    /// returns Result indicating success status of the read
    fn recv_message(&mut self, response: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.read(response)
    }

    /// Handles user request to begin authenticated session with the bank
    pub fn begin_session(&mut self, user_input: &str) {
        // verify no user is logged in
        if self.is_active_user() {
            println!("A session is currently active\n");
            return;
        }

        // create static regular expression (cached to avoid re-creations)
        lazy_static! {
            static ref RE: Regex = Regex::new("^begin-session ([a-zA-Z]+)$")
                .expect("Error while compiling begin-session regular expression");
        }

        // early exit if invalid command
        if !RE.is_match(user_input) {
            println!("Usage: begin-session <user-name>");
            println!("Note:  usernames may only contain letters\n");
            return;
        }

        // extract username
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: &str = caps.get(1).unwrap().as_str();
        // validate username length
        if username.len() > MAX_USERNAME_SIZE {
            println!(
                "Error: username must be {} characters or less\n",
                MAX_USERNAME_SIZE
            );
            return;
        }

        //
        // communicate with bank

        // construct plaintext
        // TODO: what was the 0th plaintext char for again
        let mut plaintext = [0u8; MAX_PLAINTEXT_SIZE];
        for i in 0..username.len() {
            plaintext[i + 1] = username.as_bytes()[i];
        }
        // send plaintext to bank
        while let Err(_) = self.send_message(&plaintext) {}

        // receive response
        // TODO: bank not properly sending user existance
        let mut resp = [0u8; MAX_PLAINTEXT_SIZE];
        while let Err(_) = self.recv_message(&mut resp) {}

        println!("{:?}", str::from_utf8(&resp).unwrap());
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
