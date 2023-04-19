// this is where the code gets the shittiest.. brace yourself..

use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::builder::CreateInteractionResponseData;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::utils::Colour;

pub fn run(options: &[CommandDataOption]) -> CreateInteractionResponseData {
    let title = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let description = options
        .get(1)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let color = options
        .get(2)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let embed = CreateEmbed::default()
        .title(
            if let CommandDataOptionValue::String(option_title) = title {
                option_title.to_string()
            } else {
                "".to_string()
            },
        )
        .description(
            if let CommandDataOptionValue::String(option_description) = description {
                option_description.to_string()
            } else {
                "No description!".to_string()
            },
        )
        .color(
            if let CommandDataOptionValue::String(option_color) = color {
                println!("{}", option_color.parse::<i32>().unwrap());
                Colour::from(0x494d64)
            } else {
                Colour::from(0x494d64)
            },
        )
        .to_owned();

    return CreateInteractionResponseData::default()
        .add_embed(embed)
        .to_owned();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("embed")
        .description("Create an embed!")
        .create_option(|option| {
            option
                .name("title")
                .description("title of the embed")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("description")
                .description("contents of the embed")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("color")
                .description("hex value of embed color (without the hashtag)")
                .kind(CommandOptionType::String)
        })
        /* im too dumb to implement image shit.. too bad!
        .create_option(|option| {
            option
                .name("thumbnail")
                .description("url to thumbnail")
                .kind(CommandOptionType::String)

        })
        */
}
