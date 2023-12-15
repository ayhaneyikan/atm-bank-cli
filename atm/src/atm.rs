use common::{
    crypto::{
        CryptoState, COMM_COUNTER_IDX, MAX_PLAINTEXT_SIZE, MAX_USERNAME_SIZE, MESSAGE_START_IDX,
        MESSAGE_TYPE_IDX, PIN_SIZE, PIN_START_IDX, USERNAME_START_IDX,
    },
    io::{
        create_plaintext, insert_bytes_into_plaintext, RequestType, StreamManager, AUTH_SUCCESS,
        BANK_SERVER_ADDR,
    },
};
use lazy_static::lazy_static;
use regex::Regex;

type Username = String;

enum ATMState {
    BASE,
    LOGGED(Username),
}

/// Maintains ATM state and facilitates communications with the bank
pub struct ATM {
    state: ATMState,
    stream: StreamManager,
    /// Tracks number of communications. Incremented after SEND
    comm_count: u8,
}

impl ATM {
    /// Create new ATM instance
    pub fn new() -> Self {
        Self {
            state: ATMState::BASE,
            stream: StreamManager::from_addr(BANK_SERVER_ADDR),
            comm_count: 0,
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
            ATMState::BASE => {
                "  begin-session <user-name> <PIN>\n".to_string() + "  help\n" + "  exit"
            }
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
                    self.balance();
                } else if input == "end-session" {
                    self.end_session();
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

        let mut plaintext = create_plaintext(self.comm_count, RequestType::AuthUser);
        insert_bytes_into_plaintext(&mut plaintext, username.as_bytes(), USERNAME_START_IDX);
        insert_bytes_into_plaintext(&mut plaintext, pin.as_bytes(), PIN_START_IDX);

        // TODO encrypt prior to send
        // send plaintext to bank
        self.stream.send(&plaintext);
        self.comm_count += 1;

        // receive response
        let mut response = [0u8; MAX_PLAINTEXT_SIZE];
        self.stream.receive(&mut response).unwrap();
        // TODO handle bank thread exiting due to stale connection
        if response[COMM_COUNTER_IDX] != self.comm_count {
            println!("Connection has become stale. Exiting ATM.");
            std::process::exit(1);
        }
        self.comm_count += 1;

        if response[MESSAGE_START_IDX] != AUTH_SUCCESS {
            println!("Authentication failed");
            return;
        }

        // update login state
        self.state = ATMState::LOGGED(username.to_string());
        println!("Authorization successful");
        println!("Available commands:\n{}", self.get_help_display());
    }

    /// Handles user request to retreive balance information from bank.
    /// This method can only be reached if a user is logged in.
    fn balance(&self) {}

    fn end_session(&mut self) {
        let plaintext = create_plaintext(self.comm_count, RequestType::End);
        self.stream.send(&plaintext);
        self.comm_count += 1;

        let mut response = [0u8; MAX_PLAINTEXT_SIZE];
        self.stream.receive(&mut response).unwrap();
        if response[COMM_COUNTER_IDX] != self.comm_count {
            println!("Connection has become stale. Exiting ATM.");
            std::process::exit(1);
        }
        self.comm_count += 1;

        self.state = ATMState::BASE;
        println!("Session ended");
        println!("Available commands:\n{}", self.get_help_display());
    }

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
