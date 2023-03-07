use std::io::{self, BufRead, Write, stdout};

fn print_help() {
    println!();
    println!("=============================================================================================================");
    println!();
    println!("Commands:");
    println!("HELP    - Print this help message");
    println!("EXIT    - Exit the program");
    println!();
}

fn prompt(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    let result = stdin.lock().read_line(&mut line);
    match result {
        Ok(_) => line.trim().to_ascii_uppercase(),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_input(input: String) -> bool {
    match input.as_str() {
        "EXIT" | "E" => {
            println!("Goodbye!");
            true
        },
        "HELP" | "H" => {
            print_help();
            false
        },
        _ => {
            println!("Unknown command: {}", input);
            print_help();
            false
        }
    }
}

fn main() {
    loop  {
        let input = prompt("> ");

        let exit = handle_input(input);
        if exit {
            break;
        }
    }
}
