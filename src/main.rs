use std::io::{ self, Write };
use colored::*;

mod pin_file;
mod pin_by_hash;
mod pin_json;
mod unpin_by_hash;

fn main() {
    loop {
        println!();
        println!("{}", "-----------------".bright_yellow());
        println!("{}", "Pinata SDK - Rust".bright_yellow());
        println!("{}", "-----------------".bright_yellow());
        println!();
        println!("{}", "Select an option:".bright_cyan());
        println!("{}", "-----------------".bright_cyan());
        println!();
        println!("1. Pin File");
        println!("2. Pin By Hash");
        println!("3. Pin JSON");
        println!("4. Unpin By Hash");
        println!();
        println!("{}", "-----------------".bright_yellow());
        println!("0. Exit");
        println!("{}", "-----------------".bright_yellow());
        println!();

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!();
                println!("Pin File:");
                println!("{}", "Enter the file path or (b to go back):".bright_magenta());

                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).unwrap();
                let file_path = file_path.trim();

                if file_path != "b" {
                    let mut args = Vec::new();
                    args.push(file_path.to_owned());

                    if let Err(error) = pin_file::main_with_args(args) {
                        println!();
                        eprintln!("{}: {:?}", "Error running Pin File".red(), error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "2" => {
                println!();
                println!("Pin By Hash:");
                println!("{}", "Enter the hash or (b to go back):".bright_magenta());
                let mut hash = String::new();
                io::stdin().read_line(&mut hash).unwrap();
                let hash = hash.trim();

                if hash != "b" {
                    // validate hash
                    if hash.len() != 46 {
                        println!();
                        println!("{}", "Invalid hash. Please try again.".bright_red());
                        if !continue_or_exit() {
                            break;
                        }
                    }

                    if let Err(error) = pin_by_hash::main_with_args(hash) {
                        println!();
                        eprintln!("{}: {:?}", "Error running Pin By Hash".red(), error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "3" => {
                println!();
                println!("Pin JSON:");
                println!("{}", "Enter a JSON value or (b to go back):".bright_magenta());
                let mut json = String::new();
                io::stdin().read_line(&mut json).unwrap();
                let json = json.trim();

                if json != "b" {
                    if let Err(error) = pin_json::main_with_args(json) {
                        println!();
                        eprintln!("{}: {:?}", "Error running Pin JSON".red(), error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "4" => {
                println!();
                println!("Unpin By Hash:");
                println!("{}", "Enter the hash or (b to go back):".bright_magenta());
                let mut hash = String::new();
                io::stdin().read_line(&mut hash).unwrap();
                let hash = hash.trim();

                if hash != "b" {
                    // validate hash
                    if hash.len() != 46 {
                        println!();
                        println!("{}", "Invalid hash. Please try again.".bright_red());
                        if !continue_or_exit() {
                            break;
                        }
                    }

                    if let Err(error) = unpin_by_hash::main_with_args(hash) {
                        println!();
                        eprintln!("{}: {:?}", "Error running Unpin".red(), error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "0" => {
                println!("{}", "You chose to exit. Goodbye!".bright_green());
                break;
            }
            _ => {
                println!();
                println!("{}", "Invalid choice. Please try again.".bright_red());
            }
        }
    }
}

fn continue_or_exit() -> bool {
    loop {
        println!();
        println!("Press M to menu or E to exit: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();
        match choice.as_str() {
            "m" => {
                return true;
            }
            "e" => {
                return false;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}