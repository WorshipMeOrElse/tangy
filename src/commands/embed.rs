use std::time::Duration;

use crate::Context;
use crate::Error;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
//use poise::serenity_prelude::CacheHttp;

#[poise::command(slash_command)]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Title of embed"] title: Option<String>,
    #[description = "Description/Contents of embed"] description: Option<String>,
    #[description = "Embed footer"] footer: Option<String>,
    #[description = "Hex color of embed (without hashtag)"] hex: Option<String>,
    #[description = "url of embed thumbnail"] thumbnail: Option<String>,
    #[description = "url of embed image"] image: Option<String>,
) -> Result<(), Error> {
    let embed_title = title.unwrap_or_else(|| "".to_string());
    let embed_description = description.unwrap_or_else(|| "".to_string());
    let embed_footer = footer.unwrap_or_else(|| "".to_string());

    let hex_string = hex.unwrap_or_else(|| "FFFFFF".to_string());
    let hex_slice: &str = &hex_string[..];
    let hex_number = i32::from_str_radix(hex_slice, 16);
    // random fucking numbers are #FFFFFF or white converted to i32
    let color = hex_number.unwrap_or_else(|_| 16777215);

    let embed_thumbnail = thumbnail.unwrap_or_else(|| "".to_string());
    let embed_image = image.unwrap_or_else(|| "".to_string());

    // TODO: make an embed in a separate message and just use ctx.send to confirm
    //
    ChannelId(ctx.channel_id().into()).send_message(ctx, |m|
        m.embed(|e| {
            e.title(embed_title)
                .description(embed_description)
                .color(serenity::utils::Colour::from(color))
                .footer(|f| f.text(embed_footer))
                .thumbnail(embed_thumbnail)
                .image(embed_image)
        })).await?;

    let reply = ctx.send(|m| 
        m.content("Embed sent! this message will automatically delete in 5 seconds!")).await;

    tokio::time::sleep(Duration::new(5, 0)).await;

    reply?.delete(ctx).await?;

    Ok(())
}
