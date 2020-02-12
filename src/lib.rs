use std::error::Error;
use serenity::client::Client;
use serenity::model::{
    channel::Message,
    id::{
        GuildId,
        RoleId,
        UserId
    },
    guild::{
        Role
    }
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

    match user_has_existing_color_role(ctx, msg) {
        Some(role_id) => {
            let colour = parse_color(&msg.content);

            if let Some(guild_id) = msg.guild_id {
                if let Err(e) = edit_role(ctx, &guild_id, &role_id, colour) {
                    eprintln!("Error creating role: {}",e);
                }
            }
        },
        None => {
            let name = &msg.author.name;
            let role_name = format!("{}'s color", name);
            let colour = parse_color(&msg.content);
    
            
            if let Some(guild_id) = msg.guild_id {
                match create_role(ctx, &guild_id, &role_name, colour) {
                    Ok(role) => {
                        if let Err(e) = attach_role(ctx, msg, &msg.author.id, &role.id) {
                            eprintln!("Error attaching role: {}", e);
                        }
                    },
                    Err(e) => eprintln!("Error creating role: {}", e)
                }
            }
        }
    }
    
    Ok(())
}

fn parse_color(msg: &str) -> Colour {
    Colour::DARK_RED
}

fn user_has_existing_color_role(ctx: &mut Context, msg: &Message) -> Option<RoleId> {
    if let Some(_guild) = msg.guild(&ctx) {
        let guild = _guild.read();
        let guild_roles = &guild.roles;
        if let Some(member) = &msg.member {
            let member_roles = &member.roles;
            for role_id in member_roles {
                if let Some(guild_role) = guild_roles.get(role_id) {
                    if guild_role.name.contains("color") {
                        return Some(guild_role.id);
                    }
                }
            }
        }
    }
    None
}

fn create_role(ctx: &mut Context, guild: &GuildId, name: &str, colour: Colour) -> Result<Role,Box<dyn Error>> {
    let role = guild.create_role(ctx, |r| r.hoist(true).name(name).colour(colour.0 as u64))?;
    Ok(role)
}

fn attach_role(ctx: &mut Context, msg: &Message, user_id: &UserId, role_id: &RoleId) -> Result<(),Box<dyn Error>> {
    if let Some(_guild) = msg.guild(&ctx) {
        let mut guild = _guild.write();
        let guild_members = &mut guild.members;
        if let Some(member_to_attach_role) = guild_members.get_mut(user_id) {
            member_to_attach_role.add_role(ctx, role_id)?;
        }
    }
    Ok(())
}

fn edit_role(ctx: &mut Context, guild: &GuildId, role_id: &RoleId, colour: Colour) -> Result<(),Box<dyn Error>> {
    guild.edit_role(ctx, role_id, |r| r.colour(colour.0 as u64))?;

    Ok(())
}