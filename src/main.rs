mod commands;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

// in case I actually need to store this shit
// struct UserData {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pong!";
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), commands::help::help(), commands::embed::embed()],
            ..Default::default()
        })
        .token(std::env::var("TOKEN").expect("missing bot token"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        });

    framework.run().await.unwrap();
    println!("its time to get tangy!");
}
