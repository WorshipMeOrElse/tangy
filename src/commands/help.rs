use crate::Error;
use crate::Context;

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Help not implemented yet..";
    ctx.say(response).await?;
    Ok(())
}
