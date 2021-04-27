use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub discord: Discord,
    pub counter: Counter,
    #[serde(rename = "role")]
    pub roles: Vec<Role>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Discord {
    pub application_id: u64,
    pub token: String,
    pub guild_id: u64,
    pub status: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Counter {
    pub total_member_id: u64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Role {
    pub role_id: u64,
    pub role_name: String,
    pub counter_id: u64,
}
