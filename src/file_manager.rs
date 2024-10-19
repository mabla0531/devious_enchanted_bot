use std::fs;

use crate::{errors::FileManagerError, model::Player};

pub fn player_vec_to_csv(players: Vec<Player>) -> String {
    players
        .iter()
        .map(|player| format!("{},{}", player.id, player.name))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn player_csv_to_vec(players: &str) -> Vec<Player> {
    players
        .split('\n')
        .filter_map(|player| {
            let chunks = player.split(',').collect::<Vec<&str>>();
            if chunks.len() < 2 {
                None
            } else {
                Some(Player {
                    id: chunks[0].to_string(),
                    name: chunks[1].to_string(),
                })
            }
        })
        .collect()
}

pub fn get_filepath_from_clan_id(clan_id: &str) -> String {
    format!("./clans/{}.json", clan_id)
}

pub fn read_players_from_file(clan_id: &str) -> Result<Vec<Player>, FileManagerError> {
    let player_csv = fs::read_to_string(get_filepath_from_clan_id(clan_id))
        .map_err(|e| FileManagerError::ReadClanMemberFile(e))?;

    Ok(player_csv_to_vec(&player_csv))
}

pub fn write_players_to_file(clan_id: &str, players: Vec<Player>) -> Result<(), FileManagerError> {
    let file_path = get_filepath_from_clan_id(clan_id);
    fs::create_dir_all("./clans/").map_err(|e| FileManagerError::WriteClanMemberFile(e))?;
    fs::write(file_path, player_vec_to_csv(players))
        .map_err(|e| FileManagerError::WriteClanMemberFile(e))
}
