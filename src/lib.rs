use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    CommandResult,
    StandardFramework,
    macros::{
        command,
        group
    }
};

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
    let author = msg.author;

    if let Some(guild) = msg.guild_id {
        guild.create_role(ctx, |r| r.hoist(true).name("role"));
    }
    
    Ok(())
}