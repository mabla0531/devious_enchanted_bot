use std::error::Error;

use reqwest;

use crate::{
    errors::WgApiError,
    model::{Player, Response},
};

pub async fn get_player_list_from_wg(clan_id: &str) -> Result<Vec<Player>, WgApiError> {
    let body = reqwest::get(
        format!("https://api.worldoftanks.com/wot/clans/info/?application_id=bd09ad6840803e46880ac67012edf241&clan_id={}&members_key=id", clan_id)
    ).await?
        .text()
        .await?;
    println!("body = {}", body);

    format_clan_info_string(&body)
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
