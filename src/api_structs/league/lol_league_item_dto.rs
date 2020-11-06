use serde::{Deserialize, Serialize};
use crate::api_structs::league::lol_mini_series_dto::MiniSeriesDTO;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueItemDTO {
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    pub wins: i16,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "miniSeries")]
    pub mini_series: Option<MiniSeriesDTO>,
    pub inactive: bool,
    pub veteran: bool,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
    pub rank: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i16,
    pub losses: i16,
    #[serde(rename = "summonerId")]
    summoner_id: String
}