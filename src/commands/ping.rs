use crate::Error;

#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = "pong!";
    ctx.say(response).await?;
    Ok(())
}
