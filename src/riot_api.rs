use super::api_structs::lol_api_key::LolApiKey;
use super::api_structs::lol_account::LeagueAccount;
use super::util::http_client::HttpClient;
use crate::util::http_client::HttpError;

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

    fn get_url_from_api_key_with_name(original_url : &str, lol_api_key : &LolApiKey, name : String) -> String {
        return Self::get_url_from_api_key(original_url, lol_api_key)
            .replace("%name%", name.as_str());
    }

    pub fn get_status(lol_api_key : &LolApiKey) -> Result<String, HttpError> {
        let url : String = Self::get_url_from_api_key(RiotApi::GET_STATUS_URL, lol_api_key);

        return HttpClient::get(url);
    }

    pub fn get_summoner(summoner_name : String, lol_api_key : &LolApiKey) -> Result<LeagueAccount, HttpError> {
        let url : String = Self::get_url_from_api_key_with_name(RiotApi::SUMMONER_BY_NAME_URL, lol_api_key, summoner_name);

        let http_result = HttpClient::get(url);

        if http_result.is_ok() {
            let league_account : LeagueAccount = serde_json::from_str(&http_result.unwrap()).unwrap();

            println!("++++++++++++++++++++");
            println!("{:#?}", league_account);
            println!("++++++++++++++++++++");

            return Ok(league_account);
        } else {
            return Err(http_result.unwrap_err());
        }
    }
}