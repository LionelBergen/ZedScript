use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LolPerk {
    #[serde(rename = "perkIds")]
    perk_ids: Vec<i16>,
    #[serde(rename = "perkStyle")]
    perk_style: i32,
    #[serde(rename = "perkSubStyle")]
    perk_sub_style: i32,
}

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
    pub skin_id: Option<i64>,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: Option<i64>,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub bot: bool,
    #[serde(rename = "summonerId")]
    pub summoner_id: Option<String>,
    #[serde(rename = "gameCustomizationObjects")]
    pub game_customization_objects: Option<Vec<i16>>,
    pub perks: Option<LolPerk>,
}
