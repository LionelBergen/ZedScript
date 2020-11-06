use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MiniSeriesDTO {
    pub losses: i16,
    pub progress: String,
    pub target: i16,
    pub wins: i16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueEntryDto {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i16,
    pub wins: i16,
    pub losses: i16,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
    pub veteran: bool,
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    pub inactive: bool,
    #[serde(rename = "miniSeries")]
    pub mini_series: Option<MiniSeriesDTO>
}
