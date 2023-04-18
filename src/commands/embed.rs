use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::builder::CreateInteractionResponseData;
use serenity::builder::CreateEmbed;

pub fn run(_options: &[CommandDataOption]) -> CreateInteractionResponseData {
    let embed = CreateEmbed::default().description("IM ALIVE").to_owned();
    return CreateInteractionResponseData::default().add_embed(embed).to_owned();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("embed").description("Create an embed!")
}
