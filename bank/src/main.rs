use std::{
    net::{TcpListener, TcpStream}, thread, io::{Error, Read, Write},
    io::{BufReader, self}, sync::{Arc, Mutex},
};
use pool::ThreadPool;

/* import bank structure */
mod bank;
use crate::bank::Bank;

/* define program constants */
const BANK_BINDING_ADDR: &str = "0.0.0.0:32001";

const ATM_MESSAGE_SIZE: usize = 300usize;  /* temp value for this */



fn main() {
    /* 
     * this program needs to handle local commands and remote commands
     * this is done by spawning a thread to handle local connections
     * and leaving main to listen for TCP connections and spawn threads to
     * handle those connections
     */

    /* create Bank reference */
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank::new()));

    /* spawn local command thread */
    let bank_clone: Arc<Mutex<Bank>> = bank.clone();
    let local_thread = thread::spawn(|| process_local_commands(bank_clone));

    
    // /* bind to port 32001 to listen for atm requests */
    // let listener: TcpListener = TcpListener::bind(BANK_BINDING_ADDR).expect(
    //     "Error: bank could not bind"
    // );
    
    // /* create thread pool to handle fixed maximum number of concurrent clients */
    // let pool: ThreadPool = ThreadPool::new(5);


    // let mut bank = Rc::new(Bank::new());


    // for stream in listener.incoming() {
    //     match stream {
    //         Err(e) => eprintln!("Error reading incoming stream: {}", e),
    //         Ok(stream) => {
    //             /* spawn thread to handle this connection */
    //             pool.execute(|| {
    //                 handle_connection(stream);
    //             });
    //         }
    //     }
    // }

    /* join the local thread */
    local_thread.join().expect("Join failed: local command thread has panicked");
    /* join the remote threads */

}


fn process_local_commands(bank: Arc<Mutex<Bank>>) {
    println!("Local command handler");

    /* read in user input */
    let prompt: String = String::from("BANK: ");
    let mut user_input: String = String::new();

    print!("{prompt}");             // prompt user
    io::stdout().flush().unwrap();  // flush output buffer to terminal
    while io::stdin().read_line(&mut user_input).expect("Failed to read line from stdin") > 0 {
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
        }
        else if user_input.starts_with("deposit") {
            bank.process_deposit(&user_input);
        }
        else if user_input.starts_with("balance") {
            bank.process_balance(&user_input);
        }
        else if user_input == "help" {
            println!("  create-user <user-name> <pin> <balance>");
            println!("  deposit <user-name> <amt>");
            println!("  balance <user-name>");
            println!("  close bank\n");
        }
        else {
            println!("Invalid command\n");
        }

        /* reprompt user */
        print!("{prompt}");
        io::stdout().flush().unwrap(); // flush prompt
        /* clear user input buffer before next read */
        user_input.clear();
    }
}


// /* processes a client stream */
// fn handle_connection(mut stream: TcpStream) {
//     println!("Incoming connection from: {:?}", stream.peer_addr());
    
//     /* create message buffer */
//     let mut buffer = [0u8; 512];

//     let mut reader: BufReader<TcpStream> = BufReader::with_capacity(MAX_MESSAGE_SIZE, stream);

//     loop {
//         let bytes_read: usize = stream.read(&mut buffer).unwrap_or(0usize);
//         if bytes_read == 0 {
//             return;
//         }


//     }

    
    
//     println!("{:?}", buffer);
//     stream.write(&buffer[..bytes_read]).unwrap();
// }
