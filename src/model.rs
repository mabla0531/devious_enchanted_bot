use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub status: String,
    pub data: HashMap<String, MemberList>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberList {
    pub members: HashMap<String, MemberData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberData {
    pub account_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
}
