
use reqwest;

use crate::{
    errors::WgApiError,
    model::{Lookup, Player, Response},
};

pub async fn get_clan_id_by_tag_from_wg_api(clan_tag: String) -> Result<u64, WgApiError> {
    let body = reqwest::get(
        format!("https://api.worldoftanks.com/wot/clans/list/?application_id=bd09ad6840803e46880ac67012edf241&search={}&fields=clan_id", clan_tag)
    ).await?
        .text()
        .await?;

    let clan_id = get_clan_id_from_lookup_results(&body)?;
    Ok(clan_id)
}

pub async fn get_player_list_from_wg(clan_id: u64) -> Result<Vec<Player>, WgApiError> {
    let body = reqwest::get(
        format!("https://api.worldoftanks.com/wot/clans/info/?application_id=bd09ad6840803e46880ac67012edf241&clan_id={}&members_key=id", clan_id)
    ).await?
        .text()
        .await?;

    match format_clan_info_string(&body) {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("Error deserializing response: {:?}\n\n--------PAYLOAD--------\n{}", e, body);
            Err(e)
        }
    }
}

pub fn get_clan_id_from_lookup_results(info_string: &str) -> Result<u64, WgApiError> {
    let response = serde_json::de::from_str::<Lookup>(&info_string)?;
    let clan_ids = response
        .data
        .iter()
        .map(|result| result.clan_id.clone()).collect::<Vec<u64>>();

    match clan_ids.first() {
        Some(clan_id) => Ok(*clan_id),
        None => Err(WgApiError::LookupClanIDError),
    }
}

pub fn format_clan_info_string(info_string: &str) -> Result<Vec<Player>, WgApiError> {
    let response = serde_json::de::from_str::<Response>(&info_string)?;
    let players = response
        .data
        .iter()
        .map(|body| {
            body.1
                .members
                .iter()
                .map(|member| Player {
                    id: member.0.clone(),
                    name: member.1.clone().account_name,
                })
                .collect::<Vec<Player>>()
        })
        .collect::<Vec<Vec<Player>>>()
        .concat();

    Ok(players)
}
