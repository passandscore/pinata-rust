use std::io::{ self, Write };
use std::fs;
use colored::Colorize;

mod pin_file;
mod pin_by_hash;
mod pin_json;
mod unpin_by_hash;

fn main() {
    let menu_image = fs
        ::read_to_string("ascii_image.txt")
        .expect("Failed to read ASCII image file");

    loop {
        println!();
        println!("{:^45}", menu_image);
        println!("{:^45}", "-----------------");
        println!("{:^45}", "   PINATA RUST   ");
        println!("{:^45}", "-----------------");
        println!();

        println!("{:^40}", "1. Pin File(s)");
        println!("{:^43}", "2. Pin By Hash");
        println!("{:^40}", "3. Pin JSON");
        println!("{:^45}", "4. Unpin By Hash(es)");
        println!();
        println!("{:^43}", "0. >>> Exit >>>".red());
        println!();

        print!("Select a feature: ");
        println!();

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!();
                println!("{:^10}", "Pin File:".blue());
                println!("{:^20}", "Enter the file path or (b to go back):");

                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).unwrap();
                let file_path = file_path.trim();

                if file_path != "b" {
                    // split the file_path
                    let file_path: Vec<&str> = file_path.split_whitespace().collect();

                    for file in file_path {
                        println!();
                        print!("Pinning {}... ", file);

                        if let Err(error) = pin_file::main_with_args(Some(file)) {
                            println!();
                            eprintln!("Error running Unpin: {:?}", error);
                        }
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "2" => {
                println!();
                println!("{:^10}", "Pin By Hash:".blue());
                println!("{:^20}", "Enter the hash or (b to go back):");
                let mut hash = String::new();
                io::stdin().read_line(&mut hash).unwrap();
                let hash = hash.trim();

                if hash != "b" {
                    // validate hash
                    if hash.len() != 46 {
                        println!();
                        println!("{:^20}", "Invalid hash. Please try again.");
                        if !continue_or_exit() {
                            break;
                        }
                    }

                    if let Err(error) = pin_by_hash::main_with_args(hash) {
                        println!();
                        eprintln!("Error running Pin By Hash: {:?}", error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "3" => {
                println!();
                println!("{:^10}", "Pin JSON:".blue());
                println!("{:^20}", "Enter a JSON value or (b to go back):");
                let mut json = String::new();
                io::stdin().read_line(&mut json).unwrap();
                let json = json.trim();

                if json != "b" {
                    if let Err(error) = pin_json::main_with_args(json) {
                        println!();
                        eprintln!("Error running Pin JSON: {:?}", error);
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }
            "4" => {
                println!();
                println!("{:^10}", "Unpin By Hash:".blue());
                println!("{:^20}", "Enter the hash(s) separated by spaces or (b to go back):");
                let mut hashes = String::new();
                io::stdin().read_line(&mut hashes).unwrap();
                let hashes = hashes.trim();

                if hashes != "b" {
                    // split the hashes
                    let hashes: Vec<&str> = hashes.split_whitespace().collect();

                    for hash in hashes {
                        println!();
                        print!("Unpinning {}... ", hash);
                        // validate hash
                        if hash.len() != 46 {
                            println!();
                            println!("{:^20}", "Invalid hash. Please try again.");
                            if !continue_or_exit() {
                                break;
                            }
                        }

                        if let Err(error) = unpin_by_hash::main_with_args(hash) {
                            println!();
                            eprintln!("Error running Unpin: {:?}", error);
                        }
                    }
                    if !continue_or_exit() {
                        break;
                    }
                }
            }

            "0" => {
                print_thank_you_message();
                break;
            }
            _ => {
                println!();
                println!("{:^20}", "Invalid choice. Enter 0, 1, 2, 3 or 4".red());
                if !continue_or_exit() {
                    break;
                }
            }
        }
    }
}

fn continue_or_exit() -> bool {
    loop {
        println!();
        println!("{:^20}", "Press M to menu or E to exit: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();
        match choice.as_str() {
            "m" => {
                return true;
            }
            "e" => {
                print_thank_you_message();
                return false;
            }
            _ => {
                println!();
                println!("{:^20}", "Invalid choice. Please try again.".red());
                continue;
            }
        }
    }
}

fn print_thank_you_message() {
    println!("{}", "Thank you for using Pinata-Rust!".blue());
}
