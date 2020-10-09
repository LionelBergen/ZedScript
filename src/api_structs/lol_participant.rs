use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LolParticipant {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: i64,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "skinIndex")]
    pub skin_id: i64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    #[serde(rename = "summonerName")]
    pub summoner_id: String,
    pub bot: bool,
}
