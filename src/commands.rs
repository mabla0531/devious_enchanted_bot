use poise::{reply, CreateReply, ReplyHandle};
use serenity::all::{EditMessage, Message, MessageBuilder, UserId};

use crate::{add_clan, errors::InteractionError, get_all_left, get_left, remove_clan};

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn leftall(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Getting all left players...").await?;

    match get_all_left().await {
        Ok(clan_list) => {
            let player_string: String = clan_list.iter().map(
                |clan| format!(
                    "## {}: \n{}",
                    clan.0,
                    clan.1.iter().map(
                        |player| format!("{}:\nhttps://tomato.gg/stats/NA/{}-{}", player.name, player.name, player.id)
                    ).collect::<String>()
                )).collect();

            ctx.say(format!("{}", player_string)).await?;
        },
        Err(e) => {
            println!("ERROR: {:?}", e);
            let msg = MessageBuilder::new()
                .mention(&UserId::new(781796885517959218))
                .push(" your shit broke again (what a retard lmao)")
                .build();

            ctx.say(msg).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn left(
    ctx: Context<'_>,
    #[description = "Clan tag to check"] tag: String,
) -> Result<(), Error> {
    let tag = tag.trim().to_uppercase();

    let original_msg = ctx.reply(format!("Getting left players from clan {}...", tag)).await?;

    match get_left(&tag).await {
        Ok(players) => {
            let mut player_string: String = players.iter().map(|player| format!("- {}: https://tomato.gg/stats/NA/{}-{}\n", player.name, player.name, player.id)).collect();
            if player_string.is_empty() {
                player_string = "*No players have left since last check.*".to_string();
            }
            original_msg.edit(ctx, CreateReply::default().content(format!("## {}: \n{}", tag, player_string))).await?;
        },
        Err(e) => {
            println!("ERROR: {:?}", e);
            let msg = match e {
                InteractionError::InvalidClanTag => MessageBuilder::new()
                    .mention(ctx.author())
                    .push(" Invalid clan tag :(")
                    .build(),
                _ => MessageBuilder::new()
                    .mention(&UserId::new(781796885517959218))
                    .push(" your shit broke again (what a retard lmao)")
                    .build(),
            };

            ctx.say(msg).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Clan tag to add"] tag: String,
) -> Result<(), Error> {
    let original_msg = ctx.reply(format!("Adding clan {}...", tag)).await?;

    match add_clan(&tag).await {
        Ok(_) => { original_msg.edit(ctx, CreateReply::default().content(format!("Added clan {} to list of watched clans.", tag))).await?; },
        Err(e) => {
            println!("ERROR: {:?}", e);
            let msg = match e {
                InteractionError::InvalidClanTag => MessageBuilder::new()
                    .mention(ctx.author())
                    .push(" Invalid clan tag :(")
                    .build(),
                InteractionError::AlreadyAddedError => MessageBuilder::new()
                    .mention(ctx.author())
                    .push(" Clan is already watched!")
                    .build(),
                _ => MessageBuilder::new()
                    .mention(&UserId::new(781796885517959218))
                    .push(" Your shit broke again (what a retard lmao)")
                    .build(),
            };
            ctx.say(msg).await?;
        
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Clan tag to remove"] tag: String,
) -> Result<(), Error> {
    let original_msg = ctx.reply(format!("Removing clan {}", tag)).await?;

    match remove_clan(&tag).await {
        Ok(_) => { original_msg.edit(ctx, CreateReply::default().content(format!("Removed clan {} from list of watched clans.", tag))).await?; },
        Err(e) => {
            println!("ERROR: {:?}", e);
            let msg = match e {
                InteractionError::InvalidClanTag => MessageBuilder::new()
                    .mention(ctx.author())
                    .push(" Invalid clan tag :(")
                    .build(),
                _ => MessageBuilder::new()
                    .mention(&UserId::new(781796885517959218))
                    .push(" your shit broke again (what a retard lmao)")
                    .build(),
            };
            ctx.say(msg).await?;
        }
    }

    Ok(())
}
