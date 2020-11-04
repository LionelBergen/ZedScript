use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    #[serde(rename = "summonerId")]
    summoner_id: String,
    position: String,
    role: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamDto {
    pub id: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i64,
    pub tier: i64,
    pub captain: String,
    pub abbreviation: String,
    pub players: Vec<PlayerDto>
}