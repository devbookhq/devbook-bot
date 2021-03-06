mod utils;
use std::{
    env,
    sync::Arc,
    time::Instant,
};

use string_builder::Builder;

use serenity::
{
    async_trait,
    model::
    {
        event::ResumedEvent,
        channel::
        {
            Message,
            MessageType,
            ReactionType
        },
        guild::Role,
        gateway::Ready,
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
        let msg_args:Vec<&str> = msg.content.split(" ").collect();
        let command = msg_args[0];

        if msg.kind == MessageType::MemberJoin{
            if let Err(why) = msg.react(&ctx.http, ReactionType::Unicode("👋".to_string())).await {
                println!("Error reacting to message: {:?}", why)
            }
        }
        else if command == PREFIX.to_owned()+"ping" {
            let old = Instant::now();
            let mut msg:Message = msg.channel_id.say(&ctx.http, "Ping!").await.expect("Something wrong");
            let new = Instant::now();
            let data = ctx.data.read().await;

            let shard_manager = match data.get::<ShardManagerContainer>() {
                Some(v) => v,
                None => {
                    msg.reply(&ctx, "There was a problem getting the shard manager").await.expect("Something wrong");
                    return;
                },
            };

            let manager = shard_manager.lock().await;
            let runners = manager.runners.lock().await;

            let runner = match runners.get(&ShardId(ctx.shard_id)) {
                Some(runner) => runner,
                None => {
                    msg.reply(&ctx, "No shard found").await.expect("Something wrong");
                    return;
                },
            };
            if let Err(why) = msg.edit(&ctx, |m| 
                                       m.content(
                                           format!("Https Ping: {:?}ms\nShard Ping: {:?}", (new-old).as_millis(), runner.latency.expect("Unable to get Shard ping"))
                                           )
                                      ).await{
                println!("Error editting message: {:?}", why);
            };

        }
        else if command == PREFIX.to_owned()+"invite"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "https://discord.gg/ypuZfadw8H").await{
                println!("Error responding to message: {:?}", why)
            }
        }
        else if command == PREFIX.to_owned()+"quit"{
            if !utils::is_user_mod(msg.member(&ctx.http).await.expect("Unable to get member").user.id.as_mut_u64()){
                msg.reply(&ctx.http, "This is a mod only command").await.expect("");
            }
            else{
                let data = ctx.data.read().await;
                if let Some(manager) = data.get::<ShardManagerContainer>() {
                    msg.reply(&ctx, "Shutting down!").await.expect("Unable to shutdown");
                    manager.lock().await.shutdown_all().await;
                } else {
                    msg.reply(&ctx, "There was a problem getting the shard manager").await.expect("Double Interupt");
                }
            }
        }
        else if command == PREFIX.to_owned()+"mkrole"{
            if msg_args.len() != 3 {
                msg.reply(
                    &ctx.http, format!("Usage: `{}mkrole 0x123abc Rust Dev`", PREFIX)
                    ).await.expect("Unable to send message");
            }
            else if !utils::is_user_mod(msg.member(&ctx.http).await.expect("Unable to get member").user.id.as_mut_u64()){
                msg.reply(&ctx.http, "This is a mod only command").await.expect("");
            }
            else if !(&ctx.http.get_guild(msg.guild_id.expect("Unable to retreive guild id").0).await.expect("Unable to get guild").role_by_name(msg_args[2]).is_none()){
                let pguild = &ctx.http.get_guild(msg.guild_id.expect("Unable to retreive guild id").0).await.expect("Unable to get guild");
                msg.reply(
                    &ctx.http, format!("A role with the name already exists, instead use {}assign {} to get the role", PREFIX, 
                                       pguild.role_by_name(msg_args[2]).expect("Unable to retreive role").mention().to_string()
                                      )
                    ).await.expect("Unable to send message");
            }
            else{
                let hex = msg_args[1];
                let stripped_hex = hex.strip_prefix("0x").unwrap_or(hex);
                let role_hex:u64 = u64::from_str_radix(stripped_hex, 16).expect("Input wasn't hexadecimal");
                let role:Role = msg.guild_id.expect("Unable to get guild_id").create_role(&ctx.http, |r| {
                    r.hoist(false).mentionable(true).colour(role_hex).name(msg_args[2])
                }).await.expect("Unable to create role");
                msg.reply(&ctx.http, format!("Made role {}", role.mention().to_string())).await.expect("Unable to send message");
            }
        }
        else if command== PREFIX.to_owned()+"gang"{
            let gang_role:Role = ctx.http.get_guild(787118958012661790).await.expect("Unable to get guild")
                .role_by_name("[) gang").expect("Unable to get role").clone();
            let mut member = msg.member(&ctx.http).await.expect("Unable to retreive member");
            member.add_role(&ctx.http, gang_role).await.expect("Unable to add role");
            member.edit(
                &ctx.http, |em| em.nickname( 
                    format!("[) {}", member.clone().display_name().into_owned())
                    )
                ).await.expect("Unable to access member");
        }
        else if command== PREFIX.to_owned()+"assign"{
            let mut member = msg.member(&ctx.http).await.expect("Unable to retreive member");
            let guild = &ctx.http.get_guild(msg.guild_id.expect("Unale to get guild id").0).await.expect("Unable to retreive guild");
            for i in 1..msg_args.len(){
                let r_name = msg_args[i];
                let res_role = guild.role_by_name(r_name);
                if res_role.is_none(){
                    msg.reply(&ctx.http, "No role with name found").await.expect("Unable to send message");
                }
                else{
                    let role = res_role.expect("Unable to retreive role");
                    if utils::is_assignable_role(&role.id.0){
                        msg.reply(
                            &ctx.http, format!("I just assigned @{} to @{}", role.name, member.clone().display_name().into_owned() )
                            ).await.expect("Unable to respond");
                        member.add_role(&ctx.http, role).await.expect("Unable to add role");
                    }
                    else {
                        msg.reply(
                            &ctx.http, format!("Role {} can't be added as it's private", role.mention().to_string())
                            ).await.expect("Unable to respond");
                    }
                }
            }
        }
        else if command== PREFIX.to_owned()+"unassign"{
            let mut member = msg.member(&ctx.http).await.expect("Unable to retreive member");
            let guild = &ctx.http.get_guild(msg.guild_id.expect("Unale to get guild id").0).await.expect("Unable to retreive guild");
            for i in 1..msg_args.len(){
                let r_name = msg_args[i];
                let res_role = guild.role_by_name(r_name);
                if res_role.is_none(){
                    msg.reply(&ctx.http, "No role with name found").await.expect("Unable to send message");
                }
                else{
                    let role = res_role.expect("Unable to retreive role");
                    if utils::is_assignable_role(&role.id.0){
                        msg.reply(
                            &ctx.http, format!("I just removed @{} from @{}", role.name, member.clone().display_name().into_owned())
                            ).await.expect("Unable to respond");
                        member.remove_role(&ctx.http, role).await.expect("Unable to remove role");
                    }
                    else {
                        msg.reply(
                            &ctx.http, format!("Role {} can't be removed as it's private", role.mention().to_string())
                            ).await.expect("Unable to respond");
                    }
                }
            }
        }
        else if command== PREFIX.to_owned()+"list"{
            let roles = &ctx.http.get_guild(msg.guild_id.expect("Unable to retreive guild").0).await.expect("Unable to get guild")
                .roles;
            let mut sb = Builder::default();
            for (role_id, role) in roles{
                if utils::is_assignable_role(&role_id.0){
                    sb.append(format!("@{}, ", role.name));
                }
            }
            msg.reply(&ctx.http, sb.string().expect("UTF8 Error")).await.expect("Unable to reply");
        }
        else if command== PREFIX.to_owned()+"tag"{
            let subcommand = msg_args[1];
            let res = utils::tags(subcommand);
            if res.is_none(){
                msg.reply(&ctx.http, "Tag not found").await.expect("Unable to reply");
            }else{
                msg.reply(&ctx.http, res.unwrap()).await.expect("Unable to reply");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
   async fn resume(&self, _: Context, _: ResumedEvent) {
       println!("Reconnected")
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
