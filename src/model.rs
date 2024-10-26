use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// clan tag lookup
#[derive(Deserialize, Debug, Clone)]
pub struct Lookup {
    pub status: String,
    pub data: Vec<Entry>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Entry {
    pub clan_id: u64,
}


// player lookup

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub status: String,
    pub data: HashMap<String, MemberList>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberList {
    pub members: HashMap<u32, MemberData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberData {
    pub account_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}
