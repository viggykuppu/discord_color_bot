use serenity::{framework::standard::CommandError, model::prelude::*};
use crate::color_parser;

use std::error::Error;

use log::error;
use serenity::{
    model::{
        guild::{ Role },
        id::{
            GuildId,
            RoleId,
        },
        user::User
    },
    prelude::*,
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

pub async fn set_color(ctx: &Context, color_arg: String, member: &mut Member) -> Result<(), Box<dyn Error>> {
    match color_parser::parse_color(&color_arg) {
        Ok(color) => {
            match user_has_existing_color_role(ctx, &member).await {
                Some(role_id) => {
                    if let Err(e) = update_existing_role_color(ctx, &member, &role_id, color).await {
                        error!("Update existing role failed: {}", e);
                        return Err(e);
                    }
                },
                None => {
                    if let Err(e) = create_and_attach_color_role(ctx, member, color).await {
                        error!("Create and attach new role failed: {}", e);
                        return Err(e);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Command: {}; Error: {}", color_arg, e);
            match e {
                color_parser::ColorParseError::InvalidColor => {},
                color_parser::ColorParseError::InvalidGrey => {}
            };
            return Err(CommandError::from("failed to set color"));
        }
    }
    Ok(())
}

pub async fn interaction_respond(ctx: &Context, interaction: &Interaction, msg: &str) -> Result<(),Box<dyn Error>> {
    interaction.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    .content(msg)
            })
    }).await?;

    Ok(())
}

async fn update_existing_role_color(ctx: &Context, member: &Member, role_id: &RoleId, color: Colour) -> Result<(),Box<dyn Error>> {
    edit_role(ctx, &member.guild_id, role_id, color).await?;
    Ok(())
}

async fn create_and_attach_color_role(ctx: &Context, member: &mut Member, color: Colour) -> Result<(),Box<dyn Error>> {
    let name = member.display_name();
    let role_name = format!("{}'s color", name);

    let role = create_role(ctx, &member.guild_id, &role_name, color).await?;
    attach_role(ctx, member, &role.id).await?;

    Ok(())
}

async fn user_has_existing_color_role(ctx: &Context, member: &Member) -> Option<RoleId> {
    if let Ok(guild_roles) = member.guild_id.roles(ctx).await {
        let member_roles = &member.roles;
        for role_id in member_roles {
            if let Some(guild_role) = guild_roles.get(role_id) {
                if guild_role.name.contains("color") {
                    return Some(guild_role.id);
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

async fn attach_role(ctx: &Context, member: &mut Member, role_id: &RoleId) -> Result<(),Box<dyn Error>> {
    member.add_role(ctx, role_id).await?;
    Ok(())
}