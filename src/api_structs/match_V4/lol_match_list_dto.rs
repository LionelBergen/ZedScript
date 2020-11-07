use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchListDto {
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "totalGames")]
    pub total_games: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
    pub matches: Vec<MatchReferenceDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchReferenceDto {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub role: String,
    pub season: i64,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub champion: i64,
    pub queue: i64,
    pub lane: String,
    pub timestamp: i64,
}
