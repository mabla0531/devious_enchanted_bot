pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn left(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Getting left players...").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Clan tag to add"] arg1: String,
) -> Result<(), Error> {
    ctx.say("Adding clan...").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Clan tag to remove"] arg1: String,
) -> Result<(), Error> {
    ctx.say("Removing clan...").await?;

    Ok(())
}
