mod bank;
use common::{
    crypto::{
        COMM_COUNTER_IDX, MAX_COMM_COUNTER, MAX_PLAINTEXT_SIZE, MAX_USERNAME_SIZE,
        MESSAGE_START_IDX, MESSAGE_TYPE_IDX, PIN_END_IDX, PIN_SIZE, PIN_START_IDX,
        USERNAME_END_IDX, USERNAME_START_IDX,
    },
    io::{RequestType, StreamManager, AUTH_FAILURE, AUTH_SUCCESS, BANK_SERVER_ADDR},
};
use regex::bytes;

use crate::bank::Bank;
use std::{
    io::{self, BufReader},
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::{Arc, Mutex},
    thread,
};

/// Bank entrypoint
fn main() {
    /*
     * this program needs to handle local commands and remote commands
     * this is done by spawning a thread to handle local connections
     * and leaving main to listen for remote TCP connections and spawn threads
     * to handle those connections
     */

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank::new()));

    // spawn thread to process local commands
    let bank_clone: Arc<Mutex<Bank>> = bank.clone();
    let local_thread = thread::spawn(|| process_local_commands(bank_clone));

    // bind to port 32001 to listen for atm requests
    let listener: TcpListener =
        TcpListener::bind(BANK_SERVER_ADDR).expect("Error: bank could not bind");

    let mut remote_threads = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error getting stream from listener: {}", e),
            Ok(stream) => {
                // spawn thread to handle this connection
                let bank_clone = bank.clone();
                remote_threads.push(thread::spawn(|| {
                    handle_remote_connection(bank_clone, StreamManager::from_stream(stream))
                }));
            }
        }
    }

    // join threads -> local then remotes
    local_thread
        .join()
        .expect("Join failed: local command thread has panicked");
    for t in remote_threads {
        t.join().expect("Join failed: a remote thread has panicked");
    }
}

/// Processes commands from stdin. Expected to be run in a thread and provided
/// a safe copy of an Arc reference to the Bank instance. This function
/// retreives a lock on the bank after receiving input and utilizes the Bank's
/// methods for processing requests.
fn process_local_commands(bank: Arc<Mutex<Bank>>) {
    println!("Local command handler");

    /* read in user input */
    let prompt = String::from("BANK: ");
    let mut user_input = String::new();

    print!("{prompt}"); // prompt user
    io::stdout().flush().unwrap(); // flush output buffer to terminal
    while io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line from stdin")
        > 0
    {
        /* remove newline from user input */
        user_input.pop();

        /* provide exit functionality */
        if user_input == "close bank" {
            break;
        }

        /* retreive lock on the bank */
        let mut bank = bank.lock().unwrap();
        /* check for valid command and call appropriate helper function */
        if user_input.starts_with("create-user") {
            bank.process_create_user(&user_input);
        } else if user_input.starts_with("deposit") {
            bank.process_deposit(&user_input);
        } else if user_input.starts_with("balance") {
            bank.process_balance(&user_input);
        }
        // TODO: ADD A users COMMAND TO DISPLAY ACCOUNTS
        else if user_input == "help" {
            println!("  create-user <user-name> <pin> <balance>");
            println!("  deposit <user-name> <amt>");
            println!("  balance <user-name>");
            println!("  close bank\n");
        } else {
            println!("Invalid command\n");
        }

        /* reprompt user */
        print!("{prompt}");
        io::stdout().flush().unwrap(); // flush prompt
                                       /* clear user input buffer before next read */
        user_input.clear();
    }
}

/// Handles a remote ATM's requests
fn handle_remote_connection(bank: Arc<Mutex<Bank>>, mut manager: StreamManager) {
    // create message buffer
    let mut buffer = [0u8; MAX_PLAINTEXT_SIZE];
    // tracks number of communications. Incremented after RECEIVE
    let mut comm_count: u8 = 0;

    loop {
        // check if stream closed
        if let Err(()) = manager.receive(&mut buffer) {
            return;
        }

        // check for stale communication channel
        if buffer[COMM_COUNTER_IDX] >= MAX_COMM_COUNTER {
            // TODO send stale message, close stream, end thread
            todo!();
        }
        // check for replay or drop
        if buffer[COMM_COUNTER_IDX] != comm_count {
            // TODO send stale message, close stream, end thread
            todo!();
        }

        comm_count += 1;

        let bank = bank.lock().unwrap();

        match RequestType::try_from(buffer[MESSAGE_TYPE_IDX]) {
            Ok(RequestType::AuthUser) => {
                // TODO decryption and abstract trims
                // extract username and trim null bytes
                let username = str::from_utf8(&buffer[USERNAME_START_IDX..=USERNAME_END_IDX])
                    .unwrap()
                    .trim_end_matches(|c| c == '\0');
                // extract PIN and trim null bytes
                let pin: u16 = str::from_utf8(&buffer[PIN_START_IDX..=PIN_END_IDX])
                    .unwrap()
                    .trim_end_matches(|c| c == '\0')
                    .parse()
                    .unwrap();

                // create response
                let mut response = [0u8; MAX_PLAINTEXT_SIZE];
                response[COMM_COUNTER_IDX] = comm_count;
                response[MESSAGE_START_IDX] = match bank.attempt_authentication(username, pin) {
                    true => AUTH_SUCCESS,
                    false => AUTH_FAILURE,
                };
                // send response
                // TODO encrypt response
                manager.send(&response);
                comm_count += 1;
            }
            Err(_) => (),
        }
    }
}
