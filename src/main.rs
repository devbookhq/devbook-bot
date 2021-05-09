use std::{
    env,
    sync::Arc,
    time::Instant
};

use serenity::{
    async_trait,
    model::{channel::
        {
            Message,
            MessageType,
            ReactionType},
            gateway::Ready
        },
    client::bridge::gateway::{ShardManager, ShardId, GatewayIntents},
    prelude::*,
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

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
            let data = ctx.data.read().await;

            let shard_manager = match data.get::<ShardManagerContainer>() {
                Some(v) => v,
                None => {
                    msg.reply(&ctx, "There was a problem getting the shard manager").await.unwrap();
                    return;
                },
            };

            let manager = shard_manager.lock().await;
            let runners = manager.runners.lock().await;

            let runner = match runners.get(&ShardId(ctx.shard_id)) {
                Some(runner) => runner,
                None => {
                    msg.reply(&ctx,  "No shard found").await.unwrap();
                    return;
                },
            };
            if let Err(why) = msg.edit(&ctx, |m| m.content(format!("Https Ping: {:?}\nShard Ping: {:?}", old.elapsed(), runner.latency.unwrap()))).await{
                println!("Error editting message: {:?}", why);
            };

        }
        else if msg.content == PREFIX.to_owned()+"invite"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "https://discord.gg/ypuZfadw8H").await{
                println!("Error responding to message: {:?}", why)
            }
        }
        else if msg.content == PREFIX.to_owned()+"quit"{
            let data = ctx.data.read().await;
            if let Some(manager) = data.get::<ShardManagerContainer>() {
                msg.reply(&ctx, "Shutting down!").await.unwrap();
                manager.lock().await.shutdown_all().await;
            } else {
                msg.reply(&ctx, "There was a problem getting the shard manager").await.unwrap();
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

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
