mod config;

use anyhow::{Ok, Result};
use obws::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    // Connect to the OBS instance through obs-websocket.
    let client = Client::connect(config::IP, config::PORT, Some(config::PASSWORD)).await?;
    
    let value: &str = &args[1];

    match value {
        "recording-start" => {
            client.recording().start().await?;
        },
        "recording-stop" => {
            client.recording().stop().await?;
        },
        "buffer-save" => {
            client.replay_buffer().save().await?;
        },
        "buffer-start" => {
            client.replay_buffer().start().await?;
        },
        "buffer-stop" => {
            client.replay_buffer().stop().await?;
        },
        _ => println!("No key"),
    }
    
    Ok(())
}