use common::message::constants::MAX_USERNAME_SIZE;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

/// Internally represents a user's bank data
#[derive(Debug)]
struct User {
    name: String,
    pin: u16,
    balance: f64,
}

impl User {
    fn new(name: String, pin: u16, balance: f64) -> Self {
        Self { name, pin, balance }
    }
}

/// Defines a Bank instance which stores bank information about users.
///
/// Stores users' names, pins, and balances
pub struct Bank {
    users: HashMap<String, User>,
}

impl Bank {
    /// Creates new bank instance
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    //
    // helpers

    /// Returns CLI prompt for bank
    pub fn get_prompt() -> String {
        "BANK: ".to_string()
    }
    /// Returns CLI help list
    pub fn get_help_display() -> String {
        "  create-user <user-name> <pin> <balance>\n".to_string()
            + "  deposit <user-name> <amt>\n"
            + "  balance <user-name>\n"
            + "  users\n"
            + "  exit"
    }

    /// Adds new User account to bank hashmap
    fn create_new_account(&mut self, username: &String, pin: u16, balance: f64) {
        self.users
            .insert(username.clone(), User::new(username.clone(), pin, balance));
    }
    /// Check if given username exists in bank database
    pub fn is_existing_user(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }
    /// Attempts to authenticate the given user with the given pin
    pub fn attempt_authentication(&self, username: &str, pin: u16) -> bool {
        match self.users.get(username) {
            Some(user) => user.pin == pin,
            None => false,
        }
    }

    /// Prints the state of the bank information to stdin
    fn display_users(&self) {
        println!("Bank user information:");
        for (k, v) in &self.users {
            println!("{k} -> {:?}", v);
        }
        println!();
    }

    /// Processes user input based on content
    pub fn process_input(&mut self, input: &str) {
        if input.starts_with("create-user") {
            self.process_create_user(input);
        } else if input.starts_with("deposit") {
            self.process_deposit(input);
        } else if input.starts_with("balance") {
            self.process_balance(input);
        } else if input == "users" {
            self.display_users();
        } else if input == "help" {
            println!("{}", Bank::get_help_display());
        } else {
            println!("Invalid command. Use `help` to see options.");
        }
    }

    /// Processes a request to create a new user. The given request must include
    /// a username, pin for the user, and an initial balance.
    fn process_create_user(&mut self, user_input: &str) {
        lazy_static! {
            static ref CU_RE: Regex =
                Regex::new(r"^create-user ([a-zA-Z]+) ([0-9]{4}) ([0-9]+\.?[0-9]{0,2})$")
                    .expect("Error while compiling create-user regular expression");
        }

        // ensure input matches
        if !CU_RE.is_match(user_input) {
            println!("Usage: create-user <user-name> <4-digit-PIN> <balance>\n");
            return;
        }

        // validate username, pin, and balance from matched string
        let caps: Captures = CU_RE.captures(user_input).unwrap();

        let username: String = caps.get(1).unwrap().as_str().to_string();
        if username.len() > MAX_USERNAME_SIZE {
            println!(
                "Error: username must be {} characters or less\n",
                MAX_USERNAME_SIZE
            );
            return;
        }

        // pin is guaranteed to be 4 digits
        let pin: u16 = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();

        // validate initial balance
        let balance: f64 = match caps.get(3).unwrap().as_str().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Error: we don't have a big enough vault to store a balance this large\n");
                return;
            }
        };

        // ensure this isn't a duplicate username
        if self.is_existing_user(&username) {
            println!("Error: user {} already exists\n", username);
            return;
        }

        self.create_new_account(&username, pin, balance);
        println!("Created account for {}\n", username);
    }

    /// Processes a request to make a deposit into a user's account. The given
    /// request must include a username and an amount to deposit.
    fn process_deposit(&mut self, user_input: &str) {
        lazy_static! {
            static ref D_RE: Regex = Regex::new(r"^deposit ([a-zA-Z]+) ([0-9]+\.?[0-9]{0,2})$")
                .expect("Error while compiling deposit regular expression");
        }

        // ensure input matches
        if !D_RE.is_match(user_input) {
            println!("Usage: deposit <user-name> <amount>\n");
            return;
        }

        let caps: Captures = D_RE.captures(user_input).unwrap();

        // validate username
        let username: String = caps.get(1).unwrap().as_str().to_string();
        if !self.is_existing_user(&username) {
            println!("Error: account name not recognized\n");
            return;
        }

        // validate deposit amount
        let amount: f64 = match caps.get(2).unwrap().as_str().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!(
                    "Error: we don't have a big enough vault to store wealth of this magnitute\n"
                );
                return;
            }
        };

        // retreive user
        let user_data = self.users.get_mut(&username).unwrap();

        // check for deposit overflow
        if f64::MAX - amount < user_data.balance {
            println!("Error: we would drown in money trying to process this request, which is no good for anybody\n");
            return;
        }

        user_data.balance += amount;

        println!("${:.2} was successfully deposited into the account", amount);
        println!("Balance for {} is: ${:.2}\n", username, user_data.balance);
    }

    /// Processes a request to view a user's balance
    fn process_balance(&mut self, user_input: &str) {
        lazy_static! {
            static ref B_RE: Regex = Regex::new("^balance ([a-zA-Z]+)$")
                .expect("Error while compiling balance regular expression");
        }

        // ensure input matches
        if !B_RE.is_match(user_input) {
            println!("Usage: balance <user-name>\n");
            return;
        }

        let caps: Captures = B_RE.captures(user_input).unwrap();

        // validate username
        let username: String = caps.get(1).unwrap().as_str().to_string();
        if !self.is_existing_user(&username) {
            println!("Error: account name not recognized\n");
            return;
        }

        // display user balance
        println!(
            "Balance for {} is: ${:.2}\n",
            username,
            self.users.get(&username).unwrap().balance
        );
    }
}
