use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Regex, Captures};


/* define bank constants */
const MAX_USERNAME_SIZE: usize = 250usize;

pub struct Bank {
    user_pins: HashMap<String, u16>,
    user_balances: HashMap<String, u64>,

}


impl Bank {
    /* initializer */
    pub fn new() -> Bank {
        Bank {
            user_pins: HashMap::new(),
            user_balances: HashMap::new(),
        }
    }

    /// Private utility to update Bank hashmaps with new user's pin and balance
    fn create_new_account(&mut self, username: &str, pin: u16, balance: u64) {
        self.user_pins.insert(username.to_string(), pin);
        self.user_balances.insert(username.to_string(), balance);
    }

    /// Private utility to check if given username already exists in bank
    fn is_existing_user(&self, username: &str) -> bool {
        self.user_pins.contains_key(username)
    }

    // pub fn display_users(&self) {
    //     println!("Printing state of hashmaps:");
    //     for (k, v) in &self.user_pins {
    //         println!("pins: {k} -> {v}");
    //     }
    //     for (k, v) in &self.user_balances {
    //         println!("balances: {k} -> {v}");
    //     }
    // }

    pub fn process_create_user(&mut self, user_input: &String) {
        lazy_static! {
            static ref RE: Regex = Regex::new("^create-user ([a-zA-Z]+) ([0-9]{4}) ([0-9]+)$").expect(
                "Error while compiling create-user regular expression"
            );
        }
        
        /* ensure that input matches */
        if ! RE.is_match(user_input) {
            println!("Usage: create-user <user-name> <pin> <balance>\n");
            return;
        }
        /* valid match */

        /* extract and validate username, pin, and balance from matched string */
        let caps: Captures = RE.captures(user_input).unwrap();

        let username: &str = caps.get(1).unwrap().as_str();
        if username.len() > MAX_USERNAME_SIZE {
            println!("Error: username must be {} characters or less\n", MAX_USERNAME_SIZE);
            return;
        }

        let pin: u16 = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();  // pin is guaranteed to fit within u16
        
        let balance: u64 = match caps.get(3).unwrap().as_str().parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Error: we don't have a big enough vault to store a balance this large\n");
                return;
            }
        };

        /* ensure this isn't a duplicate username */
        if self.is_existing_user(username) {
            println!("Error: user {username} already exists\n");
            return;
        }

        self.create_new_account(username, pin, balance);
        println!("Created account for {username}\n");
    }

    
    pub fn process_deposit(&mut self, user_input: &String) {
        lazy_static! {
            static ref RE: Regex = Regex::new("^deposit ([a-zA-Z]+) ([0-9]+)$").expect(
                "Error while compiling deposit regular expression"
            );
        }

        /* ensure input matches */
        if ! RE.is_match(user_input) {
            println!("Usage: deposit <user-name> <amount>\n");
            return;
        }
        /* valid match */

        /* extract username and deposit amount */
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: &str = caps.get(1).unwrap().as_str();
        let amount: u64 = match caps.get(2).unwrap().as_str().parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Error: we don't have a big enough vault to store a balance this large\n");
                return;
            }
        };

        if ! self.is_existing_user(username) {
            println!("Error: account name not recognized\n");
            return;
        }

        /* check for deposit overflow */
        if u64::MAX - amount < *self.user_balances.get(username).unwrap_or(&u64::MAX) {
            println!("Error: we would drown in money trying to process this request, which is no good for anybody\n");
            return;
        }

        /* make deposit */
        let old_balance: u64 = *self.user_balances.get(username).unwrap();
        self.user_balances.insert(username.to_string(), old_balance + amount);
        println!("${amount} was successfully deposited into {username}'s account");
        println!("Your balance is now {}\n", old_balance + amount);
    }


    pub fn process_balance(&mut self, user_input: &String) {
        println!("balance stuff beep boop\n");
    }


}
