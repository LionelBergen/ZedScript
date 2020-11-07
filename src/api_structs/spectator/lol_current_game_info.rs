use serde::{Deserialize, Serialize};
use crate::api_structs::lol_game::BannedChampion;
use crate::api_structs::spectator::lol_observer::Observer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CurrentGameInfo {
    #[serde(rename = "gameId")]
    pub match_id: i64,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "mapId")]
    pub map_id: i8,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i16,
    pub observers: Observer,
    pub participants: Vec<CurrentGameParticipant>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CurrentGameParticipant {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: i64,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "skinIndex")]
    pub skin_id: Option<i64>,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: Option<i64>,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub bot: bool,
    #[serde(rename = "summonerId")]
    pub summoner_id: Option<String>,
    #[serde(rename = "gameCustomizationObjects")]
    pub game_customization_objects: Option<Vec<GameCustomizationObject>>,
    pub perks: Option<Perk>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameCustomizationObject {
    pub category: String,
    pub content: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Perk {
    #[serde(rename = "perkIds")]
    perk_ids: Vec<i16>,
    #[serde(rename = "perkStyle")]
    perk_style: i32,
    #[serde(rename = "perkSubStyle")]
    perk_sub_style: i32,
}