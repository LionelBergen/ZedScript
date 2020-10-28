#[derive(PartialEq, Eq, Debug)]
pub enum Region {
    RU,
    KR,
    BR1,
    OCE,
    JP,
    NA,
    EUNE,
    EUW,
    TR,
    LAN,
    LAS,
}

impl Region {
    pub fn get_code(&self) -> &str {
        match self {
            Region::RU => "ru",
            Region::KR => "kr",
            Region::BR1 => "br1",
            Region::OCE => "oc1",
            Region::JP => "jp1",
            Region::NA => "na1",
            Region::EUNE => "eun1",
            Region::EUW => "euw1",
            Region::TR => "tr1",
            Region::LAN => "la1",
            Region::LAS => "la2",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Region::RU => "RU",
            Region::KR => "KR",
            Region::BR1 => "BR",
            Region::OCE => "OCE",
            Region::JP => "JP",
            Region::NA => "NA",
            Region::EUNE => "EUNE",
            Region::EUW => "EUW",
            Region::TR => "TR",
            Region::LAN => "LAN",
            Region::LAS => "LAS",
        }
    }
}
