extern crate zed_script;

use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;

#[test]
fn equality_test() {
    let x1 = LolApiKey{
        api_key: "testapikey".to_string(),
        region: Region::RU,
        fully_load_classes: false
    };

    let x2 = LolApiKey{
        api_key: "testapikey".to_string(),
        region: Region::RU,
        fully_load_classes: false
    };

    let x3 = LolApiKey{
        api_key: "testapikey".to_string(),
        region: Region::RU,
        fully_load_classes: true
    };

    assert_eq!(x1, x2);
    assert_eq!(x2, x1);

    assert_ne!(x2, x3);
    assert_ne!(x3, x1);
}