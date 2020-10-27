use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionMasteryDto {
    #[serde(rename = "championPointsUntilNextLevel")]
    pub champion_points_until_next_level: i64,
    #[serde(rename = "chestGranted")]
    pub chest_granted: bool,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "lastPlayTime")]
    pub last_play_time: i64,
    #[serde(rename = "championLevel")]
    pub champion_level: i8,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "championPoints")]
    pub champion_points: i32,
    #[serde(rename = "championPointsSinceLastLevel")]
    pub champion_points_since_last_level: i64,
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: i16
}
