use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::{prelude::*, framework::standard::Command};

// Represents serializable Discord bot configuration
#[derive(Serialize, Deserialize)]
struct DiscordConfig {
    token: String,
    prefix: String,
}

// represents serializable database configuration (sample in config.toml)
#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

// represents serializable configuration
#[derive(Serialize, Deserialize)]
struct Config {
    discord: DiscordConfig,
    database: DatabaseConfig,
}

impl  Config {
    // reads config from file
    fn read_from_file() -> Config {
        let mut file = File::open("config.toml").expect("Unable to open config.toml");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read config.toml");
        toml::from_str(&contents).expect("Unable to parse config.toml")
    }
}

// Handler for Discord events
struct Handler;
#[async_trait]
impl EventHandler for Handler {}

// create a ping command that responds with "Pong!" and latency
fn create_ping_command() -> Command {
    Command::create("ping", |ctx, msg, args| {
        let latency = ctx.http.latency();
        msg.channel_id.say(&ctx.http, format!("Pong! Latency: {}", latency))?;
        Ok(())
    })
}

#[tokio::main]
async fn main() {
    // read config from file
    let config = Config::read_from_file();

    // create client
    let mut client = Client::new(&config.discord.token, Handler).expect("Error creating client");

    // start client
    client.start().await.expect("Error starting client");
}
