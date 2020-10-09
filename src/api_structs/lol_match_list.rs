use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchList {
    pub matches: Vec<Match>,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
    #[serde(rename = "totalGames")]
    pub total_games: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub champion: i64,
    pub queue: i64,
    pub season: i64,
    pub timestamp: i64,
    pub role: String,
    pub lane: String,
}
