use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::builder::CreateInteractionResponseData;

pub fn run(_options: &[CommandDataOption]) -> CreateInteractionResponseData {
    return CreateInteractionResponseData::default().content("VIOLENCE???!").to_owned()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
