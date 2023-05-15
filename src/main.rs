use base64;
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: String,
    pubkey: String,
    created_at: u64,
    kind: u16,
    tags: Vec<Vec<String>>,
    content: String,
    sig: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("NIP-95 tool")
        .version("1.0")
        .author("0xtr <oxtrr@protonmail.com>")
        .about("Decodes base64 encoded files from nostr events")
        .arg(
            Arg::new("input")
            .short('i')
            .help("Input file that contains an array of kind 1064 kind events")
            .required(true),
        )
        .arg(
            Arg::new("output")
            .short('o')
            .help("Output directory to store the decoded files")
            .required(true),
        )
        .get_matches();

    // Get the input file and output directory from the command line arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();

    // Read the file to a string
    let file_content = fs::read_to_string(input_file).await?;

    // Parse the JSON content to a vector of Event
    let data: Vec<Event> = serde_json::from_str(&file_content)?;

    // Create the output directory
    fs::create_dir_all(output_dir).await?;

    for item in data {
        // Decode the base64 content
        let decoded_content = base64::decode(&item.content)?;

        // Find the tag with type "type"
        let filetype = item
            .tags
            .iter()
            .find(|tag| tag.get(0) == Some(&"type".to_string()))
            .and_then(|tag| tag.get(1))
            .and_then(|s| s.split('/').nth(1))
            .unwrap_or("txt");

        // Write the decoded content to a new file in the output directory
        let file_path = format!("{}/{}.{}", output_dir, item.id, filetype);
        fs::write(Path::new(&file_path), &decoded_content).await?;
    }

    println!("Decoding completed successfully!");

    Ok(())
}
