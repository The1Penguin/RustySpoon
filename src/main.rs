use std::env;
use std::fs;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
    }

    // Prints successfully connected
    async fn ready(&self, _: Context, ready: Ready) {
        println!("connected!");
    }
}

#[tokio::main]
async fn main() {
    // Extract token from env variable
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Creates a client and a handler
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    // Starts the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

