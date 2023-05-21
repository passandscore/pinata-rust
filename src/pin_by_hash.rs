use std::env;
use dotenv::dotenv;
use pinata_sdk::{ PinataApi, PinByHash };
use colored::Colorize;
use std::collections::HashMap;

#[tokio::main]
pub async fn main_with_args(hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the API key and secret from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let api_secret = env::var("PINATA_API_SECRET")?;

    // Pinata file name
    let pinata_file_name = "pin_by_hash";

    // Provide a custom file name for the pin
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), pinata_file_name.to_string());

    // Pin the file to IPFS
    let api = PinataApi::new(&api_key, &api_secret)?;
    let result = api.pin_by_hash(
        PinByHash::new(hash).set_metadata_with_name(pinata_file_name.to_string(), metadata)
    ).await;

    // Print the results
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        let link = format!("https://gateway.pinata.cloud/ipfs/{}", hash);

        println!();
        println!("{} {} {}", pinata_file_name.green(), " => ", link);
        println!();
    } else if let Err(error) = result {
        println!();
        eprintln!("{} {:?}", "Pinata API error:".red(), error);
    }

    Ok(())
}