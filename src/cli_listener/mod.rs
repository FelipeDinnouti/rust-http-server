use std::{io::{self}, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use std::thread;

fn handle_input(input: &str) -> i8 {
    if input.trim() == "exit" {
        print!("Shutting down...");
        return -1;
    }

    return 0
}


pub fn start_cli_thread(shutdown_clone: Arc<AtomicBool>) {
    print!("CLI listener thread starting...");

    // Spawn CLI listener
    thread::spawn(move || {
        use std::io::Write;
        let mut input = String::new();

        loop {
            input.clear();
            print!("> ");
            io::stdout().flush().unwrap();

            if io::stdin().read_line(&mut input).is_ok() {
                if handle_input(input.trim()) == -1 {
                    shutdown_clone.store(true, Ordering::Release);
                    break;
                }
            }
        }

        io::stdout().flush().unwrap();
    });
}