use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TournamentCodeDTO {
    pub code: String,
    #[serde(rename = "spectators")]
    pub spectators: String,
    #[serde(rename = "lobbyName")]
    pub lobby_name: String,
    #[serde(rename = "metaData")]
    pub meta_data: String,
    pub password: String,
    #[serde(rename = "teamSize")]
    pub team_size: i32,
    #[serde(rename = "providerId")]
    pub provider_id: i64,
    #[serde(rename = "pickType")]
    pub pick_type: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub id: i64,
    pub region: String,
    pub map: String,
    pub participants: Vec<String>
}
