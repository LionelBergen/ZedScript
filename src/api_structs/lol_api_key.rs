use super::lol_region::Region;

#[derive(PartialEq, Debug)]
pub struct LolApiKey {
    pub api_key: String,
    pub region: Region,
}
