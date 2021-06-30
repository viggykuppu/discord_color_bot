use std::error::Error;

use serenity::{
    client::{
        Client,
    },
    framework::standard::{
        CommandResult,
        CommandError,
        StandardFramework,
        macros::{
            command,
            group
        }
    },
    model::{
        channel::Message,
        guild::{ Role },
        id::{
            ChannelId,
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