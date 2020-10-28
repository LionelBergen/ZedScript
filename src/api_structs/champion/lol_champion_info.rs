use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionInfo {
    #[serde(rename = "maxNewPlayerLevel")]
    pub max_new_player_level: i16,
    #[serde(rename = "freeChampionIdsForNewPlayers")]
    pub free_champion_ids_for_new_players: Vec<i16>,
    #[serde(rename = "freeChampionIds")]
    pub free_champion_ids: Vec<i16>
}
