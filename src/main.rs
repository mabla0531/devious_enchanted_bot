mod bot_manager;
mod commands;
mod errors;
mod file_manager;
mod model;
mod wg_api;

use bot_manager::start_bot;
use errors::InteractionError;
use file_manager::{read_players_from_file, write_players_to_file};
use model::Player;
use wg_api::get_player_list_from_wg;

#[tokio::main]
async fn main() {
    start_bot().await.expect("Error starting bot");
}

pub async fn add_clan(clan_id: &str) -> Result<(), InteractionError> {
    let player_list = get_player_list_from_wg(clan_id).await?;
    write_players_to_file(clan_id, player_list)?;

    Ok(())
}

pub async fn remove_clan(clan_id: &str) -> Result<(), InteractionError> {
    // delete file

    Ok(())
}

pub async fn get_left(clan_id: &str) -> Result<Vec<Player>, InteractionError> {
    let current_player_list = get_player_list_from_wg(clan_id).await?;
    let mut previous_player_list = read_players_from_file(clan_id)?;
    previous_player_list.retain(|e| !current_player_list.contains(e));

    Ok(previous_player_list)
}
