use thiserror::Error;

#[derive(Error, Debug)]
pub enum InteractionError {
    #[error("Invalid clan tag")]
    InvalidClanTag,
    #[error("Clan already added")]
    AlreadyAddedError,
    #[error("Error fetching new player list: {0}")]
    GetNewPlayersError(WgApiError),
    #[error("Error getting clan id from tag: {0}")]
    GetAPIClanTagError(WgApiError),
    #[error("Error getting clan tag: {0}")]
    GetClanTagError(DBManagerError),
    #[error("Error updating players: {0}")]
    UpdatePlayersError(DBManagerError),
    #[error("Error getting players: {0}")]
    GetPlayersError(DBManagerError),
    #[error("Error adding clan: {0}")]
    AddClanError(DBManagerError),
    #[error("Error removing clan: {0}")]
    RemoveClanError(DBManagerError),
}

#[derive(Error, Debug)]
pub enum WgApiError {
    #[error("No results found for clan tag")]
    LookupClanIDError,
    #[error("Could not get player list from WG API: {0}")]
    GetPlayerListError(#[from] reqwest::Error),
    #[error("Could not parse API player payload: {0}")]
    ParsePlayerPayloadError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum DBManagerError {
    #[error("Could not init db connection: {0}")]
    InitConnectionError(rusqlite::Error),
    #[error("Could not get clan ID from db: {0}")]
    GetClanID(rusqlite::Error),
    #[error("Clan is already present")]
    ClanAlreadyPresent,
    #[error("Clan tag is not present in DB")]
    InvalidClanName,
    #[error("Could not read clan list: {0}")]
    GetClanList(rusqlite::Error),
    #[error("Could not read player list: {0}")]
    GetClanMembers(rusqlite::Error),
    #[error("Could not write player list: {0}")]
    WriteClanMembers(rusqlite::Error),
    #[error("Could not add clan: {0}")]
    AddClan(rusqlite::Error),
    #[error("Could not remove clan: {0}")]
    RemoveClan(rusqlite::Error),
}
