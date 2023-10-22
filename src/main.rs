use std::env;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready
    },
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let pong = msg.channel_id.say(&ctx, "Pong!").await;
            if let Err(why) = pong {
                println!("Error sending message: {:?}", why);
            }

            let dm = msg.author.dm(&ctx,|m| m.content(" ")).await;
            if let Err(why) = dm {
                println!("t {:?}", why)
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;





    let mut client: Client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}