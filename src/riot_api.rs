use crate::api_structs::lol_account::LeagueAccount;
use crate::api_structs::lol_api_key::LolApiKey;
use crate::api_structs::lol_game::GameList;
use crate::api_structs::lol_match_list::MatchList;
use crate::util::http_client::HttpClient;
use crate::util::http_error::HttpError;

pub struct RiotApi {}

impl RiotApi {
    const GET_STATUS_URL: &'static str =
        "https://%region%.api.riotgames.com/lol/status/v3/shard-data?api_key=%apikey%";

    // THIRD_PARTY_CODE-V4
    const GET_FEATURED_GAMES_URL: &'static str =
        "https://%region%.api.riotgames.com/lol/spectator/v4/featured-games?api_key=%apikey%";

    // MATCH-V4
    const GET_MATCHLIST_URL: &'static str = "https://%region%.api.riotgames.com/lol/match/v4/matchlists/by-account/%accountid%?api_key=%apikey%";

    // SUMMONER-V4
    const GET_SUMMONER_BY_ACCOUNT_ID: &'static str = "https://%region%.api.riotgames.com/lol/summoner/v4/summoners/by-account/%accountid%?api_key=%apikey%";
    const SUMMONER_BY_NAME_URL: &'static str = "https://%region%.api.riotgames.com/lol/summoner/v4/summoners/by-name/%name%?api_key=%apikey%";
    const SUMMONER_BY_PUIID_URL: &'static str = "https://%region%.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/%PUUID%?api_key=%apikey%";
    const SUMMONER_BY_SUMMONER_ID_URL: &'static str = "https://%region%.api.riotgames.com/lol/summoner/v4/summoners/%summonerid%?api_key=%apikey%";

    // THIRD-PARTY-CODE-V4
    const GET_THIRD_PARTY_CODE_URL: &'static str = "https://%region%.api.riotgames.com/lol/platform/v4/third-party-code/by-summoner/%summonerid%?api_key=%apikey%";

    // TOURNAMENT-V4

    pub fn get_status(lol_api_key: &LolApiKey) -> Result<String, HttpError> {
        let url: String = Self::get_url_from_api_key(RiotApi::GET_STATUS_URL, lol_api_key);

        HttpClient::get(url)
    }

    pub fn get_third_party_code(
        encrypted_summoner_id: &str,
        lol_api_key: &LolApiKey,
    ) -> Result<String, HttpError> {
        let url: String =
            Self::get_url_from_api_key(RiotApi::GET_THIRD_PARTY_CODE_URL, lol_api_key)
                .replace("%summonerid%", encrypted_summoner_id);

        HttpClient::get(url)
    }

    pub fn get_featured_games(lol_api_key: &LolApiKey) -> Result<GameList, HttpError> {
        let url: String = Self::get_url_from_api_key(RiotApi::GET_FEATURED_GAMES_URL, lol_api_key);
        let http_result = HttpClient::get(url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: GameList = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_match_list(
        account_id: &str,
        lol_api_key: &LolApiKey,
    ) -> Result<MatchList, HttpError> {
        let url: String = Self::get_url_from_api_key_with_account_id(
            RiotApi::GET_MATCHLIST_URL,
            lol_api_key,
            account_id,
        );
        let http_result = HttpClient::get(url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: MatchList = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_summoner_by_account_id(
        account_id: String,
        lol_api_key: &LolApiKey,
    ) -> Result<LeagueAccount, HttpError> {
        let url: String = Self::get_url_from_api_key_with_account_id(
            RiotApi::GET_SUMMONER_BY_ACCOUNT_ID,
            lol_api_key,
            &account_id,
        );

        Self::get_league_account_from_url(url)
    }

    pub fn get_summoner_by_puuid_id(
        puuid: String,
        lol_api_key: &LolApiKey,
    ) -> Result<LeagueAccount, HttpError> {
        let url: String = Self::get_url_from_api_key_with_puuid_id(
            RiotApi::SUMMONER_BY_PUIID_URL,
            lol_api_key,
            &puuid,
        );

        Self::get_league_account_from_url(url)
    }

    pub fn get_summoner_by_summoner_id(
        summoner_id: String,
        lol_api_key: &LolApiKey,
    ) -> Result<LeagueAccount, HttpError> {
        let url: String = Self::get_url_from_api_key_with_summoner_id(
            RiotApi::SUMMONER_BY_SUMMONER_ID_URL,
            lol_api_key,
            &summoner_id,
        );

        Self::get_league_account_from_url(url)
    }

    pub fn get_summoner(
        summoner_name: String,
        lol_api_key: &LolApiKey,
    ) -> Result<LeagueAccount, HttpError> {
        let url: String = Self::get_url_from_api_key_with_name(
            RiotApi::SUMMONER_BY_NAME_URL,
            lol_api_key,
            summoner_name,
        );

        Self::get_league_account_from_url(url)
    }

    fn get_league_account_from_url(url: String) -> Result<LeagueAccount, HttpError> {
        let http_result = HttpClient::get(url);

        match http_result {
            Ok(result) => {
                let league_account: LeagueAccount = serde_json::from_str(&result).unwrap();

                Ok(league_account)
            }
            Err(error) => Err(error),
        }
    }

    fn get_url_from_api_key(original_url: &str, lol_api_key: &LolApiKey) -> String {
        original_url
            .replace("%region%", lol_api_key.region.get_code())
            .replace("%apikey%", &*lol_api_key.api_key)
    }

    fn get_url_from_api_key_with_name(
        original_url: &str,
        lol_api_key: &LolApiKey,
        name: String,
    ) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key).replace("%name%", name.as_str())
    }

    fn get_url_from_api_key_with_account_id(
        original_url: &str,
        lol_api_key: &LolApiKey,
        account_id: &str,
    ) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key).replace("%accountid%", account_id)
    }

    fn get_url_from_api_key_with_summoner_id(
        original_url: &str,
        lol_api_key: &LolApiKey,
        account_id: &str,
    ) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key).replace("%summonerid%", account_id)
    }

    fn get_url_from_api_key_with_puuid_id(
        original_url: &str,
        lol_api_key: &LolApiKey,
        account_id: &str,
    ) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key).replace("%PUUID%", account_id)
    }
}
