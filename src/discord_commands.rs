use crate::color_parser;
use crate::bot_config;

use std::error::Error;

use log::error;
use serenity::{
    framework::standard::{
        CommandResult,
        CommandError,
    },
    model::{
        channel::Message,
        guild::{ Role },
        id::{
            GuildId,
            RoleId,
            UserId
        },
        user::User
    },
    prelude::{ Context},
    utils::Colour
};

pub async fn help(ctx: &Context, author: &User) -> Result<(),Box<dyn Error>> {
    author.dm(ctx, |m| {
        m.content(format!("Hi, I'm a bot that lets your set your name's color.
Use /setcolor <color_value> to accomplish this
Where color value is of the format #<hex_value>, <hex_value>, <decimal_value>, or <color_name>
You can find the corresponding hex values for colors here: https://www.w3schools.com/colors/colors_picker.asp
You can also find the list of supported color names here: https://www.w3schools.com/colors/colors_names.asp", ))
    }).await?;

    Ok(())
}

async fn color(ctx: &Context, msg: &Message) -> CommandResult {
    match color_parser::parse_color(&msg.content) {
        Ok(color) => {
            match user_has_existing_color_role(ctx, msg).await {
                Some(role_id) => {
                    if let Err(e) = update_existing_role_color(ctx, msg, &role_id, color).await {
                        error!("Update existing role failed: {}", e);
                    }
                },
                None => {
                    if let Err(e) = create_and_attach_color_role(ctx, msg, color).await {
                        error!("Create and attach new role failed: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            error!("Command: {}; Error: {}", msg.content, e);
            match e {
                color_parser::ColorParseError::InvalidColor => msg.reply(ctx,format!("I didn't understand the color you provided. Use the {}help command for info on what kind of colors I can accept.", bot_config::CONFIG.prefix)).await?,
                color_parser::ColorParseError::InvalidGrey => msg.reply(ctx, "I'm sorry, but I'm not allowed to use that color").await?
            };
            return Err(CommandError::from("failed to set color"));
        }
    }
    
    msg.reply(ctx, "Done!").await?;

    Ok(())
}

async fn update_existing_role_color(ctx: &Context, msg: &Message, role_id: &RoleId, color: Colour) -> Result<(),Box<dyn Error>> {
    if let Some(guild_id) = &msg.guild_id {
        edit_role(ctx, guild_id, role_id, color).await?;
    }
    Ok(())
}

async fn create_and_attach_color_role(ctx: &Context, msg: &Message, color: Colour) -> Result<(),Box<dyn Error>> {
    let name = &msg.author.name;
    let role_name = format!("{}'s color", name);

    if let Some(guild_id) = &msg.guild_id {
        let role = create_role(ctx, guild_id, &role_name, color).await?;
        attach_role(ctx, msg, &msg.author.id, &role.id).await?;
    }

    Ok(())
}

async fn user_has_existing_color_role(ctx: &Context, msg: &Message) -> Option<RoleId> {
    if let Some(_guild) = msg.guild(&ctx).await {
        // let guild = _guild.read();
        let guild_roles = _guild.roles;
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

async fn edit_role(ctx: &Context, guild: &GuildId, role_id: &RoleId, colour: Colour) -> Result<(),Box<dyn Error>> {
    guild.edit_role(ctx, role_id, |r| r.colour(colour.0 as u64)).await?;

    Ok(())
}

async fn create_role(ctx: &Context, guild: &GuildId, name: &str, colour: Colour) -> Result<Role,Box<dyn Error>> {
    let role = guild.create_role(ctx, |r| r.name(name).colour(colour.0 as u64)).await?;

    Ok(role)
}

async fn attach_role(ctx: &Context, msg: &Message, user_id: &UserId, role_id: &RoleId) -> Result<(),Box<dyn Error>> {
    if let Some(_guild) = msg.guild(&ctx).await {
        // let guild = _guild.write();
        let guild_members = &mut _guild.members(&ctx, Some(100), None).await?;
        if let Some(member_to_attach_role) = guild_members.into_iter().find(|x| x.user.id.as_u64() == user_id.as_u64()) {
            member_to_attach_role.add_role(ctx, role_id).await?;
        }
    }

    Ok(())
}