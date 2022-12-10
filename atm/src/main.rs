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
    active_user: String,

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
        active_user: String::from(""),

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
            if atm.active_user != "" {
                println!("A user is already logged in\n");
            } else {
                process_begin_session(&mut atm, &user_input);
            }
        }
        else if user_input.starts_with("withdraw") {
            /* check if there's a user logged in */
            if atm.active_user == "" {
                println!("No user logged in\n");
            } else {

            }

        }
        else if user_input == "balance" {
            /* check if there's a user logged in */
            if atm.active_user == "" {
                println!("No user logged in\n");
            } else {

            }
        }
        else if user_input == "end-session" {
            /* check if there's a user logged in */
            if atm.active_user == "" {
                println!("No user logged in\n");
            } else {
                /* someone logged in -> log them out */
                println!("{} logged out\n", atm.active_user);
                /* reset prompt and user data */
                atm.prompt = String::from("ATM: ");
                atm.active_user = String::from("");
            }
        }
        else {
            println!("Invalid command\n");
        }


        /* reprompt user */
        print!("{}", atm.prompt);
        io::stdout().flush().unwrap();
        /* clear user input buffer before next read */
        user_input.clear();
    }
}


fn process_begin_session(atm: &mut ATM, user_input: &String) {
    /* create static regular expression (creates it at most once) */
    lazy_static! {
        static ref RE: Regex = Regex::new("^begin-session ([a-zA-Z]{1,250})$").unwrap();
    }

    /* check for match */
    if RE.is_match(user_input) {
        /* extract username from matched string */
        let caps: Captures = RE.captures(user_input).unwrap();
        let username: &str = match caps.get(1) {
            Some(m) => m.as_str(),
            None => panic!("Error with match and capture"),
        };
        
        /* check if they exist communiate with bank */
        if username != "ayhan" && username != "addison" {
            println!("Not authorized\n");
            return;            
        }

        /* prompt for PIN */
        print!("  PIN: ");
        io::stdout().flush().unwrap();
        /* capture PIN */
        let mut PIN: String = String::new();
        io::stdin().read_line(&mut PIN).expect("Failed to read PIN from stdin");
        PIN.pop();  // remove newline
        /* check if provided PIN matches */
        lazy_static! {
            static ref PIN_RE: Regex = Regex::new("^[0-9]{4}$").unwrap();
        }
        if ! PIN_RE.is_match(&PIN) {
            println!("Not authorized\n");
            return;
        }

        /* validate PIN with bank */
        if PIN != "1188" {
            println!("Not authorized\n");
            return;
        }

        /* update login state */
        atm.active_user = String::from(username);
        atm.prompt = String::from("ATM (") + &username + &String::from("): ");
        println!("Authorized\n");
    }
    else {  // invalid match
        println!("Usage: begin-session <user-name>\n");
    }
}
