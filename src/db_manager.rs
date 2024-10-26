use rusqlite::{Connection, ErrorCode};

use crate::{errors::DBManagerError, model::Player};


pub struct DBHandler {
    connection: Connection,
}

unsafe impl Send for DBHandler {}
unsafe impl Sync for DBHandler {}

impl DBHandler {
    pub fn trigger_lazy_static_initialization(&self) {} // I hate it too but I'm too lazy to think

    pub fn init() -> Result<Self, DBManagerError> {
        let connection = Connection::open("./clan_data.db3") // TODO this should be done based on the guild ID supplied by discord
            .map_err(|e| DBManagerError::InitConnectionError(e))?;

        match connection.execute("CREATE TABLE IF NOT EXISTS watched_clans (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", ()) {
            Ok(_) => println!("Initialized (or skipped initializing) watched clan database"),
            Err(e) => println!("Error creating DB: {:?}", e),
        }

        Ok(Self { connection })
    }

    pub fn get_watched_clan_list(&self) -> Result<Vec<String>, DBManagerError> {
        let mut statement = self
            .connection
            .prepare("SELECT id FROM watched_clans")
            .map_err(|e| DBManagerError::GetClanList(e))?;

        let clan_id_iter = statement
            .query_map([], |row| Ok(row.get(0)?))
            .map_err(|e| DBManagerError::GetClanList(e))?;

        Ok(clan_id_iter.filter_map(|p| p.ok()).collect())
    }

    pub fn get_clan_id_from_db_by_tag(&self, clan_tag: String) -> Result<u64, DBManagerError> {
        let mut statement = self
            .connection
            .prepare(format!("SELECT id FROM watched_clans WHERE name = '{}'", clan_tag).as_str())
            .map_err(|e| DBManagerError::GetClanID(e))?;

        let clan_names: Vec<u64> = statement.query_map([], |row| Ok(row.get(0)?))
            .map_err(|e| DBManagerError::GetClanID(e))?
            .filter_map(|p| p.ok())
            .collect();

        match clan_names.len() {
            0 => Err(DBManagerError::InvalidClanName),
            _ => Ok(clan_names[0].clone())
        }
    }

    pub fn read_players_from_db(&self, clan_id: u64) -> Result<Vec<Player>, DBManagerError> {
        let mut statement = self
            .connection
            .prepare(format!("SELECT id, name FROM c{}", clan_id).as_str())
            .map_err(|e| DBManagerError::GetClanMembers(e))?;

        let player_iter = statement
            .query_map([], |row| {
                Ok(Player {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .map_err(|e| DBManagerError::GetClanMembers(e))?;

        Ok(player_iter.filter_map(|p| p.ok()).collect())
    }

    pub fn write_players_to_db(
        &self,
        clan_id: u64,
        players: Vec<Player>,
    ) -> Result<(), DBManagerError> {
        self.connection
                .execute(format!("DELETE FROM c{}", clan_id).as_str(), ())
                .map_err(|e| DBManagerError::WriteClanMembers(e))?;

        for player in players {
            self.connection
                .execute(format!("INSERT INTO c{} (id, name) VALUES ({}, '{}')", clan_id, player.id, player.name).as_str(), ())
                .map_err(|e| DBManagerError::WriteClanMembers(e))?;
        }

        Ok(())
    }

    pub fn add_clan_to_db(&self, clan_id: u64, clan_tag: &str) -> Result<(), DBManagerError> {
        self.connection
            .execute(
                format!(
                    "CREATE TABLE IF NOT EXISTS c{} (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                    clan_id
                )
                .as_str(),
                (),
            )
            .map_err(|e| DBManagerError::AddClan(e))
            .map(|_| ())?;

        match self.connection.execute(format!("INSERT INTO watched_clans (id, name) VALUES ({}, '{}')", clan_id, clan_tag).as_str(), ()) {
            Ok(_) => Ok(()),
            Err(e) => match e.sqlite_error_code() {
                Some(inner) => match inner {
                    ErrorCode::ConstraintViolation => Err(DBManagerError::ClanAlreadyPresent),
                    _ => Err(DBManagerError::AddClan(e))
                },
                None => Err(DBManagerError::AddClan(e))
            }
        }
    }

    pub fn remove_clan_from_db(&self, clan_id: u64) -> Result<(), DBManagerError> {
        self.connection
            .execute(format!("DROP TABLE c{}", clan_id).as_str(), ())
            .map_err(|e| DBManagerError::RemoveClan(e))
            .map(|_| ())
    }
}
