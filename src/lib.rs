use std::error::Error;
use serenity::client::Client;
use serenity::model::{
    channel::Message,
    id::GuildId
};
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    CommandResult,
    StandardFramework,
    macros::{
        command,
        group
    }
};
use serenity::utils::Colour;

#[group]
#[commands(ping,color)]
struct General;

struct Handler;

impl EventHandler for Handler {}

pub fn init(token: &String) -> Client {
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP));
    client
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn color(ctx: &mut Context, msg: &Message) -> CommandResult {
    let name = "test-role";
    let colour = Colour::BLITZ_BLUE;

    if let Some(guild_id) = msg.guild_id {
        if let Err(e) = create_role(ctx, guild_id, name, colour) {
            eprintln!("Error creating role: {}",e);
        }
    }
    
    Ok(())
}

fn create_role(ctx: &mut Context, guild: GuildId, name: &str, colour: Colour) -> Result<(),Box<dyn Error>> {
    guild.create_role(ctx, |r| r.hoist(true).name(name).colour(colour.0 as u64))?;

    Ok(())
}