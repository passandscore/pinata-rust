use std::env;
use dotenv::dotenv;
use pinata_sdk::{ PinataApi, PinByJson };
use colored::Colorize;
use serde_json;
use std::collections::HashMap;

#[tokio::main]
pub async fn main_with_args(json: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the API key and secret from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let api_secret = env::var("PINATA_API_SECRET")?;

    // Create an instance of PinataApi
    let api = PinataApi::new(&api_key, &api_secret)?;

    // Pinata file name
    let pinata_file_name = "pin_json";

    // Convert the provided JSON string to a HashMap
    let json_data: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;

    // Provide a custom file name for the pin
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), pinata_file_name.to_string());

    // Add each JSON field to the metadata HashMap
    for (field, value) in json_data.iter() {
        metadata.insert(field.clone(), value.to_string());
    }

    // Pin the JSON data to IPFS
    let result = api.pin_json(
        PinByJson::new(json).set_metadata_with_name(pinata_file_name, metadata)
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