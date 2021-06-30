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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction)  {
        // println!("{:?}",interaction);
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
                if let Some(InteractionData::ApplicationCommand(command)) = &interaction.data {
                    handle_command(&ctx, &interaction, command).await;
                }
            }
            _ => ()
        }
    }
}

async fn handle_command(ctx: &Context, interaction: &Interaction, command: &ApplicationCommandInteractionData) {
    let result = match command.name.as_str() { 
        "help" => handle_help(ctx, interaction).await,
        _ => Ok(())
    };
    if let Err(e) = result {
        error!("handling interaction failed {}", e);
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
        interaction.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Check your DMs 🙃"))
        }).await?;
    }
    Ok(())
}

async fn handle_color(ctx: &Context, interaction: &Interaction) -> Result<(),Box<dyn Error>> {
    let mut user: Option<&User> = None;
    if let Some(member) = &interaction.member {
        user = Some(&member.user);
        
    } else if let Some(u) = &interaction.user {
        user = Some(&u);
    };
    if let Some(u) = user {
        discord_commands::help(ctx, u).await?;
        interaction.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Check your DMs 🙃"))
        }).await?;
    }
    Ok(())
}