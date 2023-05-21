use std::env;
use dotenv::dotenv;
use pinata_sdk::PinataApi;
use colored::Colorize;

#[tokio::main]
pub async fn main_with_args(hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the API key and secret from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let api_secret = env::var("PINATA_API_SECRET")?;

    // Create an instance of PinataApi
    let api = PinataApi::new(&api_key, &api_secret)?;

    // Unpin the content
    let unpin_result = api.unpin(hash).await;

    // Print the unpin result
    match unpin_result {
        Ok(_) => {
            println!();
            println!("{}", "Successfully unpinned.".green());
            println!();
        }
        Err(error) => {
            println!();
            eprintln!("Failed to unpin content: {:?}", error);
        }
    }

    Ok(())
}