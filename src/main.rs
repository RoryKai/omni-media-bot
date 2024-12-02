use std::env;
use serenity::prelude::*;
pub mod event;
pub mod commands;
use event::Handler;
pub mod api_handler;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    // Powershell Env Setup => $env:DISCORD_TOKEN = "{token}"
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let shard_count = env::var("SHARD_COUNT")
        .expect("Expected shard count in the environment")
        .parse()
        .unwrap();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start shard & listen for events | Automatic reconnect w/ exponential backoff
    if let Err(why) = client.start_shards(shard_count).await {
        println!("Client error: {why:?}");
    }
}