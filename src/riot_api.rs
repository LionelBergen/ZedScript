use super::api_structs::lol_api_key::LolApiKey;
use super::util::http_client::HttpClient;

pub struct RiotApi {
}

impl RiotApi {
    const SUMMONER_BY_NAME_URL: &'static str = "https://%region%.api.riotgames.com/lol/summoner/v4/summoners/by-name/%name%?api_key=%apikey%";
    const GET_STATUS_URL : &'static str = "https://%region%.api.riotgames.com/lol/status/v3/shard-data?api_key=%apikey%";

    fn get_url_from_api_key(original_url : &str, lol_api_key : &LolApiKey) -> String {
        return original_url
            .replace("%region%", lol_api_key.region.get_code())
            .replace("%apikey%", &*lol_api_key.api_key);
    }

    pub fn get_status(lol_api_key : &LolApiKey) {
        let url : String = Self::get_url_from_api_key(RiotApi::GET_STATUS_URL, lol_api_key);
        HttpClient::get(url);


        //return resp.get(""); // String::from(resp);
    }
}