use poise::serenity_prelude as serenity;
use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};
use std::{error::Error, fs::read_to_string};

use crate::commands;

pub async fn start_bot() -> Result<(), Box<dyn Error>> {
    let token = read_to_string(".env").expect("Environment variable file is not present");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::left(), commands::add(), commands::remove()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client?.start().await?;

    Ok(())
}
