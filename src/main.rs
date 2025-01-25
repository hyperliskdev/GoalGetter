use clap::{Arg, Command};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;


const BASE_URL: &str = "https://statsapi.web.nhl.com/api/v1";




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // Create a new reqwest client.
    let client = Client::new();
    


    
    Ok(())
}