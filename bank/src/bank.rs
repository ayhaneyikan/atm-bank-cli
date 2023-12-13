use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

/* define bank constants */
const MAX_USERNAME_SIZE: usize = 250usize;

/// Defines a Bank instance which stores bank information about users.
///
/// Stores users' names, pins, and balances
pub struct Bank {
    users: HashMap<String, User>,
}

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

impl Bank {
    /* initializer */
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// Private utility to update Bank hashmaps with new user's pin and balance
    fn create_new_account(&mut self, username: &String, pin: u16, balance: f64) {
        self.users
            .insert(username.clone(), User::new(username.clone(), pin, balance));
    }

    /// Helper to check if given username exists in bank database
    pub fn is_existing_user(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }
    pub fn is_valid_pin(&self, username: &str, pin: u16) -> bool {
        match self.users.get(username) {
            Some(u) => u.pin == pin,
            None => false,
        }
    }

    /// Prints the state of the bank information to stdin
    pub fn display_users(&self) {
        println!("Bank user information:");
        for (k, v) in &self.users {
            println!("{k} -> {:?}", v);
        }
        println!();
    }

    /// Processes a request to create a new user. The given request must include
    /// a username, pin for the user, and an initial balance.
    pub fn process_create_user(&mut self, user_input: &String) {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^create-user ([a-zA-Z]+) ([0-9]{4}) ([0-9]+\.?[0-9]{0,2})$")
                    .expect("Error while compiling create-user regular expression");
        }

        /* ensure that input matches */
        if !RE.is_match(user_input) {
            println!("Usage: create-user <user-name> <pin> <balance>\n");
            return;
        }
        /* valid match */

        /* extract and validate username, pin, and balance from matched string */
        let caps: Captures = RE.captures(user_input).unwrap();

        let username: String = caps.get(1).unwrap().as_str().to_string();
        if username.len() > MAX_USERNAME_SIZE {
            println!(
                "Error: username must be {} characters or less\n",
                MAX_USERNAME_SIZE
            );
            return;
        }

        let pin: u16 = caps.get(2).unwrap().as_str().parse::<u16>().unwrap(); // pin is guaranteed to fit within u16

        let balance: f64 = match caps.get(3).unwrap().as_str().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Error: we don't have a big enough vault to store a balance this large\n");
                return;
            }
        };

        /* ensure this isn't a duplicate username */
        if self.is_existing_user(&username) {
            println!("Error: user {username} already exists\n");
            return;
        }

        self.create_new_account(&username, pin, balance);
        println!("Created account for {username}\n");
    }

    /// Processes a request to make a deposit into a user's account. The given
    /// request must include a username and an amount to deposit.
    pub fn process_deposit(&mut self, user_input: &String) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^deposit ([a-zA-Z]+) ([0-9]+\.?[0-9]{0,2})$")
                .expect("Error while compiling deposit regular expression");
        }

        /* ensure input matches */
        if !RE.is_match(user_input) {
            println!("Usage: deposit <user-name> <amount>\n");
            return;
        }
        /* valid match */

        /* extract username and deposit amount */
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: String = caps.get(1).unwrap().as_str().to_string();
        let amount: f64 = match caps.get(2).unwrap().as_str().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!(
                    "Error: we don't have a big enough vault to store wealth of this magnitute\n"
                );
                return;
            }
        };

        /* ensure this user exists */
        if !self.is_existing_user(&username) {
            println!("Error: account name not recognized\n");
            return;
        }

        /* retreive user */
        let mut user_data = self.users.get_mut(&username).unwrap();

        /* check for deposit overflow */
        if f64::MAX - amount < user_data.balance {
            println!("Error: we would drown in money trying to process this request, which is no good for anybody\n");
            return;
        }

        /* make deposit */
        user_data.balance += amount;

        println!(
            "${:.2} was successfully deposited into {username}'s account",
            amount
        );
        println!("Your balance is now ${:.2}\n", user_data.balance);
    }

    /// Processes a request to view a user's balance. The given request must
    /// include a username.
    pub fn process_balance(&mut self, user_input: &String) {
        lazy_static! {
            static ref RE: Regex = Regex::new("^balance ([a-zA-Z]+)$")
                .expect("Error while compiling balance regular expression");
        }

        /* ensure input matches */
        if !RE.is_match(user_input) {
            println!("Usage: balance <user-name>\n");
            return;
        }
        /* valid match */

        /* extract username */
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: String = caps.get(1).unwrap().as_str().to_string();

        /* check if recognized username */
        if !self.is_existing_user(&username) {
            println!("Error: account name not recognized\n");
            return;
        }

        /* display user balance */
        println!(
            "Balance for {} is: ${:.2}\n",
            username,
            self.users.get(&username).unwrap().balance
        );
    }
}
