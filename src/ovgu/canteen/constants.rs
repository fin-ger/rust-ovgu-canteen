macro_rules! base_url {
    [$sub:tt] => {
        concat!("https://www.studentenwerk-magdeburg.de/mensen-cafeterien/", $sub)
    }
}

#[macro_export]
macro_rules! ovgu_canteen_url {
    [unten] => ( base_url!["mensa-unicampus/speiseplan-unten/"] )
}
