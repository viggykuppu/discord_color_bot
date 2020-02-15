#[macro_use]
extern crate lazy_static;

mod color_parser;
mod color_name_map;
pub mod bot_config;

use std::error::Error;
use serenity::client::Client;
use serenity::model::{
    channel::Message,
    id::{
        ChannelId,
        GuildId,
        RoleId,
        UserId
    },
    gateway::{Ready, Activity },
    guild::{ Role }
};
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    CommandResult,
    CommandError,
    StandardFramework,
    macros::{
        command,
        group
    }
};
use serenity::utils::Colour;
use log::error;

#[group]
#[commands(color, help)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.set_activity(Activity::listening(&format!("{}help", &bot_config::CONFIG.prefix)))
    }
}

pub fn run() -> Result<(), serenity::Error> {
    let token = bot_config::CONFIG.token.clone();
    let prefix = bot_config::CONFIG.prefix.clone();
    let channel_ids = bot_config::CONFIG.channel_ids.iter().map(|id| ChannelId::from(*id) ).collect();

    let mut client = Client::new(token, Handler) 
    .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(&prefix).allowed_channels(channel_ids))
        .group(&GENERAL_GROUP));
        
    client.start()
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.author.dm(ctx, |m| {
        m.content(format!("Hi, I'm a bot that sets the color of your name!
Please enter a command of the following format:
    {}color <color_value>
Where color value is of the format #<hex_value>, <hex_value>, <decimal_value>, or <color_name>
You can find the corresponding hex values for colors here: https://www.w3schools.com/colors/colors_picker.asp
You can also find the list of supported color names here: https://www.w3schools.com/colors/colors_names.asp", bot_config::CONFIG.prefix))
    })?;

    Ok(())
}

#[command]
#[aliases(colour)]
fn color(ctx: &mut Context, msg: &Message) -> CommandResult {
    match color_parser::parse_color(&msg.content) {
        Ok(color) => {
            match user_has_existing_color_role(ctx, msg) {
                Some(role_id) => {
                    if let Err(e) = update_existing_role_color(ctx, msg, &role_id, color) {
                        error!("Update existing role failed: {}", e);
                    }
                },
                None => {
                    if let Err(e) = create_and_attach_color_role(ctx, msg, color) {
                        error!("Create and attach new role failed: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            error!("Command: {}; Error: {}", msg.content, e);
            match e {
                color_parser::ColorParseError::InvalidColor => msg.reply(ctx,format!("I didn't understand the color you provided. Use the {}help command for info on what kind of colors I can accept.", bot_config::CONFIG.prefix))?,
                color_parser::ColorParseError::InvalidGrey => msg.reply(ctx, "I'm sorry, but I'm not allowed to use that color")?
            };
            return Err(CommandError::from("failed to set color"));
        }
    }
    
    msg.reply(ctx, "Done!")?;

    Ok(())
}

fn update_existing_role_color(ctx: &mut Context, msg: &Message, role_id: &RoleId, color: Colour) -> Result<(),Box<dyn Error>> {
    if let Some(guild_id) = &msg.guild_id {
        edit_role(ctx, guild_id, role_id, color)?;
    }
    Ok(())
}

fn create_and_attach_color_role(ctx: &mut Context, msg: &Message, color: Colour) -> Result<(),Box<dyn Error>> {
    let name = &msg.author.name;
    let role_name = format!("{}'s color", name);

    if let Some(guild_id) = &msg.guild_id {
        let role = create_role(ctx, guild_id, &role_name, color)?;
        attach_role(ctx, msg, &msg.author.id, &role.id)?;
    }

    Ok(())
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

fn edit_role(ctx: &mut Context, guild: &GuildId, role_id: &RoleId, colour: Colour) -> Result<(),Box<dyn Error>> {
    guild.edit_role(ctx, role_id, |r| r.colour(colour.0 as u64))?;

    Ok(())
}

fn create_role(ctx: &mut Context, guild: &GuildId, name: &str, colour: Colour) -> Result<Role,Box<dyn Error>> {
    let role = guild.create_role(ctx, |r| r.name(name).colour(colour.0 as u64))?;

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