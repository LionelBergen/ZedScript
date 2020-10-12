extern crate zed_script;

use zed_script::api_structs::lol_account::LeagueAccount;

#[test]
fn equality_test() {
    let x1 = LeagueAccount {
        summoner_id: "123456".to_string(),
        account_id: "222".to_string(),
        puuid: "".to_string(),
        name: "username".to_string(),
        profile_icon_id: 0,
        revision_date: 0,
        summoner_level: 0,
    };

    let x2 = LeagueAccount {
        summoner_id: "123456".to_string(),
        account_id: "222".to_string(),
        puuid: "".to_string(),
        name: "username".to_string(),
        profile_icon_id: 0,
        revision_date: 0,
        summoner_level: 0,
    };

    let x3 = LeagueAccount {
        summoner_id: "different Id".to_string(),
        account_id: "111111".to_string(),
        puuid: "".to_string(),
        name: "different username".to_string(),
        profile_icon_id: 0,
        revision_date: 0,
        summoner_level: 0,
    };

    assert_eq!(x1, x2);
    assert_eq!(x2, x1);
    assert_ne!(x2, x3);
    assert_ne!(x3, x1);
}
