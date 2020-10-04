//use riot_api;
extern crate zed_script;

use zed_script::riot_api;
use zed_script::api_structs::lol_api_key::LolApiKey;

#[test]
fn it_adds_two() {
    let lol_api_key = LolApiKey {api_key: String::from(""), region: String::from(""), fully_load_classes: true};
    assert_eq!("foo", riot_api::RiotApi::get_status(&lol_api_key));
}