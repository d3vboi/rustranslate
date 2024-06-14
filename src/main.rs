use clap::{Arg, Command};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read, Write}; // Import Write trait

#[derive(Serialize)]
struct DeeplRequest {
    text: Vec<String>,
    target_lang: String,
}

#[derive(Deserialize)]
struct DeeplTranslation {
    detected_source_language: String,
    text: String,
}

#[derive(Deserialize)]
struct DeeplResponse {
    translations: Vec<DeeplTranslation>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the CLI arguments
    let matches = Command::new("rustranslate")
        .version("0.1.0")
        .author("d3vboi@proton.me")
        .about("Translates text using the DeepL API")
        .arg(
            Arg::new("set-api-key")
                .long("set-api-key")
                .help("Prompt to set the DeepL API key"),
        )
        .get_matches();

    // Check if --set-api-key was provided
    if matches.contains_id("set-api-key") {
        print!("Enter your DeepL API key: ");
        io::stdout().flush()?; // Use flush from Write trait
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key)?;
        let api_key = api_key.trim();

        // Set the environment variable
        env::set_var("DEEPL_API_KEY", api_key);

        println!("API key set successfully.");
        return Ok(());
    }

    // Read the API key from the environment variable
    let api_key = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set. Use --set-api-key to set it.");

    // Read text from STDIN
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Prepare the request payload
    let request = DeeplRequest {
        text: vec![input.trim().to_string()],
        target_lang: "DE".to_string(),
    };

    // Create an HTTP client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post("https://api.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    // Parse the response
    let deepl_response: DeeplResponse = response.json().await?;

    // Print the translations
    for translation in deepl_response.translations {
        println!("Detected source language: {}", translation.detected_source_language);
        println!("Translation: {}", translation.text);
    }

    Ok(())
}
