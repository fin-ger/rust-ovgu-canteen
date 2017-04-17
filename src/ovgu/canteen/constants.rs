macro_rules! base_url {
    [$sub:tt] => {
        concat!("https://www.studentenwerk-magdeburg.de/mensen-cafeterien/", $sub)
    }
}

#[macro_export]
macro_rules! ovgu_canteen_url {
    [unten] => ( base_url!["mensa-unicampus/speiseplan-unten/"] )
}

macro_rules! parse_and_unwrap_selector {
    ($sel:tt) => ( ::scraper::Selector::parse($sel).unwrap() )
}

#[macro_export]
macro_rules! ovgu_canteen_selector {
    [day] => ( parse_and_unwrap_selector!(".mensa > table") );
    [date] => ( parse_and_unwrap_selector!("thead > tr > td") );
    [side_dishes] => ( parse_and_unwrap_selector!("tbody > tr:last-of-type > td") );
    [meal] => ( parse_and_unwrap_selector!("tbody > tr:not(:last-of-type)") );
    [name] => ( parse_and_unwrap_selector!("td:nth-of-type(1) > strong") );
    [price] => ( parse_and_unwrap_selector!("td:nth-of-type(1)") );
    [symbols] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div > img") );
    [notes] => ( parse_and_unwrap_selector!("td:nth-of-type(2) > div:nth-of-type(2)") );
}
