use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio, Child}; // [cite: 2]

fn main() {
    loop {
        print!("my_shell> ");
        io::stdout().flush().unwrap_or_else(|e| { // [cite: 2]
            eprintln!("Error flushing stdout: {}", e);
        });

        let mut input = String::new(); // [cite: 2]
        match io::stdin().read_line(&mut input) { // [cite: 2]
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\nExiting my_shell.");
                break;
            }
            Ok(_) => {
                let trimmed_input = input.trim();

                if trimmed_input.is_empty() {
                    continue;
                }

                if trimmed_input == "exit" {
                    println!("Exiting my_shell.");
                    break;
                }

                // Handle commands, including potential pipes
                //execute_pipeline(trimmed_input);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                // Optionally break or decide to continue
                // break;
            }
        }
    }
}


