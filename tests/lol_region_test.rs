extern crate zed_script;

use zed_script::api_structs::lol_region::Region;

#[test]
fn get_code_test() {
    assert_eq!("ru", Region::RU.get_code());
    assert_eq!("kr", Region::KR.get_code());
    assert_eq!("la2", Region::LAS.get_code());
}

#[test]
fn equality_test() {
    assert_eq!(Region::RU, Region::RU);
    assert_ne!(Region::RU, Region::KR);
}
