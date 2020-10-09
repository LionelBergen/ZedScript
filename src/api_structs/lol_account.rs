use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueAccount {
    pub id: String,
    #[serde(rename="accountId")]
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    #[serde(rename="profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename="revisionDate")]
    pub revision_date: i64,
    #[serde(rename="summonerLevel")]
    pub summoner_level: i32
}

impl PartialEq for LeagueAccount {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.account_id == other.account_id && self.name == other.name;
    }
}