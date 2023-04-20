use crate::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Title of embed"] title: Option<String>,
    #[description = "Description/Contents of embed"] description: Option<String>,
    #[description = "Embed footer"] footer: Option<String>,
    #[description = "Hex color of embed (without hashtag)"] hex: Option<String>,
) -> Result<(), Error> {
    let embed_title = title.unwrap_or_else(|| "".to_string());
    let embed_description = description.unwrap_or_else(|| "".to_string());
    let embed_footer = footer.unwrap_or_else(|| "".to_string());
    let hex_string = hex.unwrap_or_else(|| "FFFFFF".to_string());
    let hex_slice: &str = &hex_string[..];

    let hex_number = i32::from_str_radix(hex_slice, 16);

    // random fucking numbers are #FFFFFF or white converted to i32
    let color = hex_number.unwrap_or_else(|_| 16777215);

    ctx.send(
        |m| m.embed(
            |e| e.title(embed_title)
                 .description(embed_description)
                 .color(serenity::utils::Colour::from(color))
                 //.thumbnail("https://media.discordapp.net/attachments/1055541263757234199/1098370280730132550/New_Project.png?width=662&height=662")
                 .footer(|f| f.text(embed_footer))
        )
    ).await?;

    Ok(())
}
