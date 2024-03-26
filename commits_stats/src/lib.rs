use std::collections::HashMap;
use chrono::{Datelike, DateTime};
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    data.members()
        .map(|commit| commit["commit"]["author"]["date"].as_str().unwrap())
        .map(|date_str| DateTime::parse_from_rfc3339(date_str).unwrap())
        .map(|date| date.iso_week())
        .for_each(|week| *hm.entry(format!("{}-W{}", week.year(), week.week())).or_insert(0) += 1 );
    hm
}
pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    data.members()
        .map(|commit| commit["author"]["login"].as_str().unwrap())
        .for_each(|author| *hm.entry(author.to_string()).or_insert(0) += 1 );
    hm
}