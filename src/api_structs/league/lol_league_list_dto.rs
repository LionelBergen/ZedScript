use serde::{Deserialize, Serialize};
use crate::api_structs::league::lol_league_item_dto::LeagueItemDTO;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueListDTO {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    pub entries: Vec<LeagueItemDTO>,
    pub tier: String,
    pub name: String,
    pub queue: String
}