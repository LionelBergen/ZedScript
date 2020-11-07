use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BannedChampion {
    #[serde(rename = "championId")]
    pub champion_id: i16,
    #[serde(rename = "teamId")]
    pub team_id: Option<i16>,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i8,
}