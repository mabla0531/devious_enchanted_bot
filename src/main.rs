mod bot_manager;
mod commands;
mod db_manager;
mod errors;
mod model;
mod wg_api;

use std::collections::HashMap;

use bot_manager::start_bot;
use db_manager::DBHandler;
use errors::{DBManagerError, InteractionError};
use model::Player;
use wg_api::{get_clan_id_by_tag_from_wg_api, get_player_list_from_wg};

use lazy_static::lazy_static;

lazy_static! {
    static ref db_handler: DBHandler = DBHandler::init().unwrap();
}

#[tokio::main]
async fn main() {
    db_handler.trigger_lazy_static_initialization();
    start_bot().await.expect("Error starting bot");
}

pub async fn get_clan_id_by_tag(clan_name: &str) -> Result<u64, InteractionError> {

    let clan_name = clan_name.to_uppercase();

    match db_handler.get_clan_id_from_db_by_tag(clan_name.clone()) {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("Error getting clan tag {} from database: \n\t{}\nchecking WG API...", clan_name, e);
            Ok(
                get_clan_id_by_tag_from_wg_api(clan_name)
                    .await
                    .map_err(|e| InteractionError::GetAPIClanTagError(e))?
            )
        }
    }
}

pub async fn add_clan(clan_name: &str) -> Result<(), InteractionError> {
    let clan_id = get_clan_id_by_tag(clan_name).await?;

    let player_list = get_player_list_from_wg(clan_id).await.map_err(|e| InteractionError::GetNewPlayersError(e))?;
    
    match db_handler.add_clan_to_db(clan_id, clan_name) {
        Ok(_) => Ok(()),
        Err(DBManagerError::ClanAlreadyPresent) => Err(InteractionError::AlreadyAddedError),
        Err(e) => Err(InteractionError::AddClanError(e))
    }?;
    
    db_handler
        .write_players_to_db(clan_id, player_list)
        .map_err(|e| InteractionError::UpdatePlayersError(e))
}

pub async fn remove_clan(clan_name: &str) -> Result<(), InteractionError> {
    let clan_id = get_clan_id_by_tag(clan_name).await?;

    db_handler
        .remove_clan_from_db(clan_id)
        .map_err(|e| InteractionError::RemoveClanError(e))
}

pub async fn get_left(clan_name: &str) -> Result<Vec<Player>, InteractionError> {
    let clan_id = get_clan_id_by_tag(clan_name).await?;
    
    let current_player_list = get_player_list_from_wg(clan_id)
        .await
        .map_err(|e| InteractionError::GetNewPlayersError(e))?;

    let mut previous_player_list = db_handler
        .read_players_from_db(clan_id)
        .map_err(|e| InteractionError::GetPlayersError(e))?;
    
    previous_player_list
        .retain(|e| !current_player_list
        .contains(e));

    db_handler
        .write_players_to_db(clan_id, current_player_list)
        .map_err(|e| InteractionError::UpdatePlayersError(e))?;

    Ok(previous_player_list)
}

pub async fn get_all_left() -> Result<HashMap<String, Vec<Player>>, InteractionError> {    
    let clan_list = db_handler
        .get_watched_clan_list()
        .map_err(|e| InteractionError::GetPlayersError(e))?;

    let mut players: HashMap<String, Vec<Player>> = HashMap::new();
    for clan in clan_list {
        players.insert(clan.clone(), get_left(clan.as_str()).await?);
    }

    Ok(players)
}
