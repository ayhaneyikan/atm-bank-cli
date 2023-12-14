mod atm;
use crate::atm::ATM;
use common::io::BANK_SERVER_ADDR;
use std::io::{self, Write};

/// ATM entrypoint
fn main() {
    let mut atm = ATM::new(BANK_SERVER_ADDR);

    // print initial prompt and flush buffer to terminal
    println!("\nAvailable commands:\n{}", atm.get_help_display());
    print!("\n{}", atm.get_prompt());
    io::stdout().flush().unwrap();

    // user input buffer
    let mut user_input = String::new();

    // iteratively read user input
    while io::stdin().read_line(&mut user_input).unwrap() > 0 {
        // remove newline from end of user input
        user_input.pop();

        if user_input == "exit" {
            break;
        }

        atm.process_input(&user_input.trim());

        // reprompt user
        print!("\n{}", atm.get_prompt());
        io::stdout().flush().unwrap();
        // clear user input buffer before next read
        user_input.clear();
    }
}
