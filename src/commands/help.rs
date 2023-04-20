use crate::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        |m| m.embed(
            |e| e.title("Help")
                 .description("This is a discord bot designed to give the end user as much freedom as they want with decorating their sever! this was originally made to spite Mee6 because you need to fucking pay to make more than one embed fuck you mee6 fuck you fuck you fuck you fuck you")
                 .color(serenity::utils::Colour::from(0xf5a97f))
                 .thumbnail("https://media.discordapp.net/attachments/1055541263757234199/1098370280730132550/New_Project.png?width=662&height=662")
                 .footer(|f| f.text("Thumbnail/Icon by popicless! Made with love but most importantly spite :)"))
        )
    ).await?;

    Ok(())
}
