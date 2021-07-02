use std::error::Error;
use log::error;

use crate::discord_commands;

use serenity::{
    async_trait,
    model::{
        interactions::{
            Interaction, 
            InteractionData,
            ApplicationCommandInteractionData,
            InteractionResponseType,
            InteractionType
        },
        gateway::{Ready, Activity },
        user::User
    },
    prelude::{EventHandler, Context}
};

pub struct ColorBotHandler;
#[async_trait]
impl EventHandler for ColorBotHandler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::listening("Mindfully Colouring")).await;
    }

    async fn interaction_create(&self, ctx: Context, mut interaction: Interaction)  {
        match interaction.kind {
            InteractionType::Ping => { 
                if let Err(e) = interaction.create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::Pong)
                })
                .await {
                    error!("Error ponging {}", e);
                };
            },
            InteractionType::ApplicationCommand => {
                if let Some(InteractionData::ApplicationCommand(_)) = interaction.data {
                    handle_command(&ctx, &mut interaction).await;
                }
            }
            _ => ()
        }
    }
}

async fn handle_command(ctx: &Context, interaction: &mut Interaction) {
    if let Some(InteractionData::ApplicationCommand(command)) = interaction.data.clone() {
        let result = match command.name.as_str() { 
            "help" => handle_help(ctx, interaction).await,
            "setcolor" => handle_color(ctx, interaction, command).await,
            _ => Ok(())
        };
        if let Err(e) = result {
            error!("handling interaction failed {}", e);
        }
    }
}

async fn handle_help(ctx: &Context, interaction: &Interaction) -> Result<(),Box<dyn Error>> {
    let mut user: Option<&User> = None;
    if let Some(member) = &interaction.member {
        user = Some(&member.user);
        
    } else if let Some(u) = &interaction.user {
        user = Some(&u);
    };
    if let Some(u) = user {
        discord_commands::help(ctx, u).await?;
        discord_commands::interaction_respond(ctx, interaction, "Check your DMs 🙃").await?;
    }
    Ok(())
}

async fn handle_color(ctx: &Context, interaction: &mut Interaction, command: ApplicationCommandInteractionData) -> Result<(),Box<dyn Error>> {
    if let Some(member) = &mut interaction.member {
        let arg = command.options[0].value.as_ref();
        let color_arg = arg.unwrap().as_str().unwrap().to_string();
        let mut error = false;
        if let Err(_) = discord_commands::set_color(ctx, color_arg, member).await {
            error = true;
        }
        if error {
            discord_commands::interaction_respond(ctx, &interaction, 
                "Beep boop I am now become error. Please tell my owner 🙃").await?;
        }
        discord_commands::interaction_respond(ctx, &interaction, 
            "Successfully done 🙃").await?;
    } else {
        discord_commands::interaction_respond(ctx, &interaction, 
            "Please use the /setcolor command on a specific server; it doesn't work in DMs 🙃").await?;
        error!("Failed to get member");
    };
    Ok(())
}

