use std::{
    env,
    sync::Arc,
    time::Instant
};

use serenity::{
    async_trait,
    model::{channel::
            {Message,
            MessageType,
            ReactionType},
        gateway::Ready
    },
    gateway::Shard,
    client::bridge::gateway::{ShardManager, ShardId, GatewayIntents},
    prelude::*,
};

struct Handler;

const PREFIX:&str = "!";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.kind == MessageType::MemberJoin{
            if let Err(why) = msg.react(&ctx.http, ReactionType::Unicode("👋".to_string())).await {
                println!("Error reacting to message: {:?}", why)
            }
        }
        else if msg.content == PREFIX.to_owned()+"ping" {
            let old = Instant::now();
            let mut msg:Message = msg.channel_id.say(&ctx.http, "Ping!").await.unwrap();
            let new = Instant::now();

            if let Err(why) = msg.edit(&ctx, |m| m.content(format!("Https Ping {}ms\nGateway Ping {}ms", (new - old).as_millis(), "{}"))).await{
                println!("Error editting message: {:?}", why);
            };

        }
        else if msg.content == PREFIX.to_owned()+"invite"{
             if let Err(why) = msg.channel_id.say(&ctx.http, "https://discord.gg/ypuZfadw8H").await{
                 println!("Error responding to message: {:?}", why)
             }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("No .env file found");
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
