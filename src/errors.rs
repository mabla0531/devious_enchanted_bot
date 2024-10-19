use thiserror::Error;

#[derive(Error, Debug)]
pub enum InteractionError {
    #[error("")]
    GetPlayerListError(#[from] WgApiError),
    #[error("Error writing clan file: {0}")]
    WritePlayersToFileError(#[from] FileManagerError),
}

#[derive(Error, Debug)]
pub enum WgApiError {
    #[error("Could not get player list from WG API: {0}")]
    GetPlayerListError(#[from] reqwest::Error),
    #[error("Could not parse API player payload: {0}")]
    ParsePlayerPayloadError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum FileManagerError {
    #[error("Could not read player list from stored CSV: {0}")]
    ReadClanMemberFile(std::io::Error),
    #[error("Could not write player list to stored CSV: {0}")]
    WriteClanMemberFile(std::io::Error),
}
