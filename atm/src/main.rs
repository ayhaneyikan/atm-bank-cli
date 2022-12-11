/*
 * ATM entry point
 */

use std::{env, io::{self, Write}, default};
use lazy_static::lazy_static;
use regex::{Regex, Captures};
use crypto;


/*
 * defines the ATM struct
 */

struct ATM {
    prompt: String,
    active_user: Option<String>,

    /* networking state */

    /* cryptographic state */
}


fn main() {
    // retreive command line argument iterator and convert to vec
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("The ATM expects exactly one argument: a \"<>.atm\" file. Found {} arguments instead", args.len());
    }
    
    /* initialize atm struct */
    let mut atm: ATM = ATM {
        prompt: String::from("ATM: "),
        active_user: None,
    };

    /* read in user input */
    let mut user_input: String = String::new();
    
    print!("{}", atm.prompt);  // initial prompt
    io::stdout().flush().unwrap();  // flush output buffer to terminal
    while io::stdin().read_line(&mut user_input).expect("Failed to read line from stdin") > 0 {
        /* remove newline from user input */
        user_input.pop();
        /* provide exit functionality */
        if user_input == "exit" {
            break;
        }


        /* check for valid command and call appropriate helper function */
        if user_input.starts_with("begin-session") {
            /* check if there's a user logged in */
            match atm.active_user {
                Some(_) => println!("A user is already logged in\n"),
                None => process_begin_session(&mut atm, &user_input),
            }
        }
        else if user_input.starts_with("withdraw") {
            /* check if there's a user logged in */
            match atm.active_user {
                Some(_) => process_withdraw(&mut atm, &user_input),
                None => println!("No user logged in\n"),
            }
        }
        else if user_input == "balance" {
            /* check if there's a user logged in */
            match atm.active_user {
                Some(_) => process_balance(&mut atm, &user_input),
                None => println!("No user logged in\n"),
            }
        }
        else if user_input == "end-session" {
            /* check if there's a user logged in */
            match atm.active_user {
                None => println!("No user logged in\n"),
                Some(username) => {
                    /* log this user out */
                    println!("{} logged out\n", username);
                    /* reset prompt and user data */
                    atm.prompt = String::from("ATM: ");
                    atm.active_user = None;
                },
            }
        }
        else {
            println!("Invalid command\n");
        }


        /* reprompt user */
        print!("{}", atm.prompt);
        io::stdout().flush().unwrap(); // flush prompt
        /* clear user input buffer before next read */
        user_input.clear();
    }
}


fn process_begin_session(atm: &mut ATM, user_input: &String) {
    /* create static regular expression (creates it at most once) */
    lazy_static! {
        static ref RE: Regex = Regex::new("^begin-session ([a-zA-Z]+)$").expect(
            "Error while compiling begin-session regular expression"
        );
    }
    /* define maximum allowed username size */
    const MAX_USERNAME_SIZE: usize = 250usize;

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
    atm.active_user = Some(String::from(username));
    atm.prompt = String::from("ATM (") + &username + &String::from("): ");
    println!("Authorized\n");
}


fn process_withdraw(atm: &mut ATM, user_input: &String) {
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


fn process_balance(atm: &mut ATM, user_input: &String) {
    /* no regular expression needed */

    /* retreive balance from the bank */
}
