use std::env;
use dotenv::dotenv;
use std::path::Path;
use pinata_sdk::{ PinataApi, PinByFile };
use colored::Colorize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the API key and secret from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let api_secret = env::var("PINATA_API_SECRET")?;

    // Read the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    let file_path: &String = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("{} Please provide a file path as an argument.", "Error:".red());
            return Ok(());
        }
    };

    // Get the file name
    let file_name: String = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string(); // Convert &&str to String

    // Provide a custom file name for the pin
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), file_name.clone());

    // Pin the file to IPFS
    let api = PinataApi::new(&api_key, &api_secret)?;
    let result = api.pin_file(
        PinByFile::new(file_path).set_metadata_with_name(file_name.clone(), metadata)
    ).await;

    // Print the results
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        let link = format!("https://gateway.pinata.cloud/ipfs/{}", hash);

        println!();
        println!("{}", "-".repeat(80).green());
        println!("{} {} {}", file_name.green(), " => ", link);
        println!("{}", "-".repeat(80).green());
        println!();
    } else if let Err(error) = result {
        eprintln!("{} {:?}", "Pinata API error:".red(), error);
    }

    Ok(())
}