use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TournamentPhaseDto {
    pub id: i64,
    #[serde(rename = "registrationTime")]
    registration_time: i64,
    #[serde(rename = "startTime")]
    start_time: i64,
    cancelled: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TournamentDto {
    pub id: i64,
    #[serde(rename = "themeId")]
    pub theme_id: i64,
    #[serde(rename = "nameKey")]
    pub name_key: String,
    #[serde(rename = "nameKeySecondary")]
    pub name_key_secondary: String,
    pub schedule: Vec<TournamentPhaseDto>
}