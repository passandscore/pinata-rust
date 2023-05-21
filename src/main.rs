use std::io::{ self, Write };

mod pin_file;
mod pin_by_hash;
mod pin_json;
mod unpin_by_hash;

fn main() {
    loop {
        println!("Select a crate to run:");
        println!("1. Pin File");
        println!("2. Pin By Hash");
        println!("3. Pin JSON");
        println!("4. Unpin By Hash");
        println!("0. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter the file path:");
                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).unwrap();
                let file_path = file_path.trim();

                let mut args = Vec::new();
                args.push(file_path.to_owned());

                if let Err(error) = pin_file::main_with_args(args) {
                    eprintln!("Error running Pin File: {:?}", error);
                }
            }
            "2" => {
                println!("Enter the hash:");
                let mut hash = String::new();
                io::stdin().read_line(&mut hash).unwrap();
                let hash = hash.trim();

                if let Err(error) = pin_by_hash::main_with_args(hash) {
                    eprintln!("Error running Pin By Hash: {:?}", error);
                }
            }
            "3" => {
                println!("Enter a JSON value:");
                let mut json = String::new();
                io::stdin().read_line(&mut json).unwrap();
                let json = json.trim();

                if let Err(error) = pin_json::main_with_args(json) {
                    eprintln!("Error running Pin JSON: {:?}", error);
                }
            }
            "4" => {
                println!("Enter the hash:");
                let mut hash = String::new();
                io::stdin().read_line(&mut hash).unwrap();
                let hash = hash.trim();

                if let Err(error) = unpin_by_hash::main_with_args(hash) {
                    eprintln!("Error running Unpin: {:?}", error);
                }
            }
            "0" => {
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}