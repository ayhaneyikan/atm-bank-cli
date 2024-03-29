mod bank;
use crate::bank::Bank;
use common::{
    io::{errors::ReceiveError, StreamManager, BANK_SERVER_ADDR},
    message::{MessageType, Plaintext},
};
use std::{
    io::{self, Write},
    net::TcpListener,
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
    // initial prompt
    println!("\nAvailable commands:\n{}", Bank::get_help_display());
    print!("\n{}", Bank::get_prompt());
    io::stdout().flush().unwrap();

    // user input buffer
    let mut user_input = String::new();

    // iteratively read user input
    while io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line from stdin")
        > 0
    {
        // remove newline from user input
        user_input.pop();

        if user_input == "exit" {
            break;
        }

        // retreive lock on the bank
        let mut bank = bank.lock().unwrap();

        bank.process_input(&user_input.trim());

        // reprompt user
        print!("\n{}", Bank::get_prompt());
        io::stdout().flush().unwrap();
        // clear input buffer before next read
        user_input.clear();
    }
}

/// Handles a remote ATM's requests
fn handle_remote_connection(bank: Arc<Mutex<Bank>>, mut manager: StreamManager) {
    // tracks number of communications
    let mut comm_count: u8 = 0;

    loop {
        // receive response and handle possible errors
        let response = match manager.receive(&mut comm_count) {
            Err(ReceiveError::EndOfStream) => return,
            Err(_) => {
                // // send stale response and exit
                // let response = Plaintext::new(&mut comm_count, MessageType::Stale);
                // manager.send_plaintext(response);
                return;
            }
            Ok(response) => response,
        };

        let bank = bank.lock().unwrap();

        match response.get_type() {
            MessageType::AuthUser => {
                let username = match response.get_user() {
                    Err(_) => return,
                    Ok(username) => username,
                };
                let pin = match response.get_pin() {
                    Err(_) => return,
                    Ok(pin) => pin,
                };
                // send auth response indicating success
                let mut plaintext = Plaintext::new(&mut comm_count, MessageType::AuthResult);
                plaintext.set_auth_result(bank.attempt_authentication(&username, pin));
                manager.send_plaintext(plaintext);
            }
            MessageType::Balance => {
                dbg!("balance");
                let username = match response.get_user() {
                    Err(_) => return,
                    Ok(username) => username,
                };
                dbg!(&username);
                let balance = match bank.get_balance(&username) {
                    Err(_) => return,
                    Ok(balance) => balance,
                };
                dbg!(balance);
                // send balance back
                let mut plaintext = Plaintext::new(&mut comm_count, MessageType::Balance);
                plaintext.set_balance(balance);
                manager.send_plaintext(plaintext);
            }
            MessageType::End => {
                let plaintext = Plaintext::new(&mut comm_count, MessageType::End);
                manager.send_plaintext(plaintext);
            }
            _ => todo!(),
        }
    }
}
