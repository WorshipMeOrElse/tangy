mod commands;

use std::env;

use serenity::async_trait;
use serenity::builder::CreateInteractionResponseData;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("DEBUG: cmd received: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "embed" => commands::embed::run(&command.data.options),
                _ => CreateInteractionResponseData::default().content("not implemented :(").to_owned()
            };

            if let Err(why) = command
                .create_interaction_response(&context.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|r| {*r = content; r})
                })
                .await
            {
                println!("Cannot respose to slash command: {}", why)
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "no.").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _guild_commands =
            Command::create_global_application_command(&context.http, |command| {
                commands::ping::register(command);
                commands::embed::register(command)
            })
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

