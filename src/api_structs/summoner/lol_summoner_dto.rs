use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerDTO {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "revisionDate")]
    pub revision_date: i64,
    pub name: String,
    #[serde(rename = "id")]
    pub summoner_id: String,
    pub puuid: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i32
}

impl PartialEq for SummonerDTO {
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id && self.name == other.name
    }
}
