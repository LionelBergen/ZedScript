use serde::{Deserialize, Serialize};
use crate::api_structs::spectator::lol_banned_champion::BannedChampion;
use crate::api_structs::spectator::lol_observer::Observer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeaturedGames {
    #[serde(rename = "gameList")]
    pub game_list: Vec<FeaturedGameInfo>,
    #[serde(rename = "clientRefreshInterval")]
    pub client_refresh_interval: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeaturedGameInfo {
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
    #[serde(rename = "mapId")]
    pub map_id: i8,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename = "gameId")]
    pub match_id: i64,
    pub observers: Observer,
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i64,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    pub participants: Vec<Participant>,
    #[serde(rename = "platformId")]
    pub platform_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Participant {
    pub bot: bool,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: i64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: Option<i64>,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: i64
}