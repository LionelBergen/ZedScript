use crate::api_structs::lol_participant::LolParticipant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameList {
    #[serde(rename = "gameList")]
    pub game_list: Vec<Game>,
    #[serde(rename = "clientRefreshInterval")]
    pub client_refresh_interval: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameObserver {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "mapId")]
    pub map_id: i8,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i16,
    pub participants: Vec<LolParticipant>,
    pub observers: GameObserver,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameTypeConfigId")]
    pub game_type_config: i64,
    // TODO: figure out what this returns
    //#[serde(rename="bannedChampions")]
    //pub banned_champions: Vec,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
}
