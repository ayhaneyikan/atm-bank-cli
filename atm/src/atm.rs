use std::io::{self, Write};
use regex::{Regex, Captures};
use lazy_static::lazy_static;

/* define atm constants */
const MAX_USERNAME_SIZE: usize = 250usize;


/*
 * defines the ATM struct
 */
pub struct ATM {
    prompt: String,
    active_user: Option<String>,

    /* networking state */

    /* cryptographic state */
    //secret_key
}


impl ATM {
    /* initializer */
    pub fn new() -> ATM {
        ATM {
            prompt: String::from("ATM: "),
            active_user: None,
        }
    }


    /* small helper functions */
    pub fn get_prompt(&self) -> &String {
        &self.prompt
    }

    /* updates active user and prompt */
    fn login_user(&mut self, username: &str) {
        self.active_user = Some(String::from(username));
        self.prompt = String::from("ATM (") + username + "): ";
    }

    fn logout_user(&mut self) {
        self.active_user = None;
        self.prompt = String::from("ATM: ");
    }

    fn is_logged_in(&self) -> bool {
        if let None = self.active_user { false } else { true }
    }



    /* attempt to begin a user session */
    pub fn process_begin_session(&mut self, user_input: &String) {
        /* verify that no user is already logged in */
        if self.is_logged_in() {
            println!("A user is already logged in\n");
            return
        }

        /* create static regular expression (creates it at most once) */
        lazy_static! {
            static ref RE: Regex = Regex::new("^begin-session ([a-zA-Z]+)$").expect(
                "Error while compiling begin-session regular expression"
            );
        }
    
        /* ensure input matches */
        if ! RE.is_match(user_input) {
            println!("Usage: begin-session <user-name>\n");
            return;
        }
        /* valid match */
    
        /* extract username from matched string */
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: &str = caps.get(1).unwrap().as_str();
        /* check if username is within max size */
        if username.len() > MAX_USERNAME_SIZE {
            println!("Error: username must be {} characters or less\n", MAX_USERNAME_SIZE);
            return;
        }
        
        /* check if they exist communiate with bank */    /* TEMP */
        if username != "ayhan" && username != "addison" {
            println!("No such user\n");
            return;            
        }
    
        /* prompt for PIN */
        print!("  PIN: ");
        io::stdout().flush().unwrap(); // flush prompt
        /* capture PIN */
        let mut pin: String = String::new();
        io::stdin().read_line(&mut pin).expect("Failed to read PIN from stdin");
        pin.pop();  // remove newline
        /* check if provided PIN matches */
        let failed_authorization: String = String::from("Not authorized\n");
        lazy_static! {
            static ref PIN_RE: Regex = Regex::new("^[0-9]{4}$").unwrap();
        }
        if ! PIN_RE.is_match(&pin) {
            println!("{}", failed_authorization);
            return;
        }
    
        /* validate PIN with bank */                     /* TEMP */
        if pin != "1188" {
            println!("{}", failed_authorization);
            return;
        }
    
        /* update login state */
        self.login_user(username);
        println!("Authorized\n");
    }


    pub fn process_withdraw(&mut self, user_input: &String) {
        /* verify there is a user logged in */
        if ! self.is_logged_in() {
            println!("No user logged in\n");
            return;
        }

        /* create static regular expression (creates it at most once) */
        lazy_static! {
            static ref RE: Regex = Regex::new("^withdraw ([0-9]+)$").expect(
                "Error while compiling withdraw regular expression"
            );
        }
    
        /* ensure input matches */
        if ! RE.is_match(user_input) {
            println!("Usage: withdraw <amt>\n");
            return;
        }
    
        /* extract withdrawal amount from input */
        let caps: Captures = RE.captures(user_input).unwrap();
        let amount_string: &str = caps.get(1).unwrap().as_str();
        
        /* check if valid u64 */
        let amount: u64;
        if let Ok(extracted_value) = amount_string.parse::<u64>() {
            amount = extracted_value;
        } else {
            println!("Error: your requested withdrawal amount is too large for our wee little bank to handle\n");
            return;
        }
        
        /* attempt withdrawal from bank */              /* TEMP */
        if amount > 1_000_000u64 {
            println!("Insufficient funds\n");
            return;
        }
        println!("${} dispensed\n", amount);
    }
    
    
    pub fn process_balance(&mut self) {
        /* verify there is a user logged in */
        if ! self.is_logged_in() {
            println!("No user logged in\n");
            return;
        }

        /* no regular expression needed */
    
        /* retreive balance from the bank */
    }

    pub fn process_end_session(&mut self) {
        /* check if there's a user logged in */
        if ! self.is_logged_in() {
            println!("No user logged in\n");
            return;
        }

        println!("{} was logged out\n", self.active_user.as_ref().expect(
            "Error: active user should never be none here"
        ));
        self.logout_user();
    }
}
