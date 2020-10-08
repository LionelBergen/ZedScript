extern crate zed_script;

use zed_script::riot_api;
use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;
use std::error::Error;

use zed_script::util::http_client::HttpError;
use std::env;

fn get_league_api_token() -> String {
    return String::from(&env::var("LEAGUE_API_KEY").unwrap());
}

/**
 * To avoid rate limiting
*/
fn pause_execution

#[test]
fn get_status_unauthorized() {
    // no api key specified, expect unauthorized response
    let lol_api_key = LolApiKey {api_key: String::from(""), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);
    let expected = HttpError{ errorMessage: String::from("Unauthorized access to application"), httpResponseCode: Some(401) };

    assert_eq!(Err(expected), result);
}

#[test]
fn get_status() {
    let lol_api_key = LolApiKey {api_key: get_league_api_token(), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);

    assert!(result.unwrap().contains("active"));
    std::thread::sleep(std::time::Duration::from_secs(1));
}