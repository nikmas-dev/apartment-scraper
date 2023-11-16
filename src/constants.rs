pub const MAX_NUMBER_OF_TRIES: u8 = 10;
pub const LINK: &str = r#"https://market.lun.ua/uk/search?currency=UAH&floor_max=5&geo_id=1&is_without_fee=false&price_max=10000&price_sqm_currency=UAH&section_id=2&sort=relevance&sub_geo_id=31117&sub_geo_id=31904"#;
pub const SELECTOR: &str = r#".feed-layout__item-holder"#;
pub const SECS_BETWEEN_REQUESTS: u64 = 5 * 60;
pub const SECS_TO_SLEEP_AFTER_REQUEST_ERROR: u64 = 10;
