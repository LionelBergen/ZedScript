use super::api_structs::lol_api_key::LolApiKey;

pub struct RiotApi {

}

impl RiotApi {
    pub fn get_status(lol_api_key : &LolApiKey) -> String {
        return String::from("foo");
    }
}