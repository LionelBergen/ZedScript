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
pub struct BannedChampion {
    #[serde(rename = "championId")]
    pub champion_id: i16,
    #[serde(rename = "teamId")]
    pub team_id: Option<i16>,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    #[serde(rename = "gameId")]
    pub match_id: i64,
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
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
}

// TODO:
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameParticipant {
"participantId":1,
"teamId":100,
"championId":145,
"spell1Id":7,
"spell2Id":4,
"stats":{
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameResult {
    #[serde(rename = "gameId")]
    pub match_id: i64,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameCreation")]
    pub game_creation: i64,
    #[serde(rename = "gameDuration")]
    game_duration: i32,
    #[serde(rename = "queueId")]
    queue_id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i8,
    #[serde(rename = "seasonId")]
    season_id: i16,
    #[serde(rename="gameVersion")]
    game_version: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    pub teams: Vec<TeamResult>,

    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i16,
    pub participants: Vec<LolParticipant>,
    pub observers: GameObserver,
    #[serde(rename = "gameTypeConfigId")]
    pub game_type_config: Option<i64>,
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TeamResult {
    #[serde(rename = "teamId")]
    pub team_id: i16,
    pub win: String,
    #[serde(rename = "firstBlood")]
    pub first_blood: bool,
    #[serde(rename = "firstTower")]
    pub first_tower: bool,
    #[serde(rename = "firstInhibitor")]
    pub first_inhibitor: bool,
    #[serde(rename = "firstBaron")]
    pub first_baron: bool,
    #[serde(rename = "firstDragon")]
    pub first_dragon: bool,
    #[serde(rename = "firstRiftHerald")]
    pub first_rift_herald: bool,
    #[serde(rename = "teamKills")]
    pub team_kills: i16,
    #[serde(rename = "towerKills")]
    pub tower_kills: i16,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i16,
    #[serde(rename = "baronKills")]
    pub baron_kills: i16,
    #[serde(rename = "dragonKills")]
    pub dragon_kills: i16,
    #[serde(rename = "vilemawKills")]
    pub vilemaw_kills: i16,
    #[serde(rename = "riftHeraldKills")]
    pub rift_herald_kills: i16,
    #[serde(rename = "dominionVictoryScore")]
    pub dominion_victory_score: i16,
    pub bans: Vec<BannedChampion>,
}