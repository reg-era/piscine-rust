use std::collections::HashMap;

use chrono::*;
use json::JsonValue;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut all_date: Vec<String> = Vec::new();

    if let JsonValue::Array(arr) = data {
        for item in arr.iter() {
            let commit = &item["commit"]["author"];
            let date = commit["date"].as_str().unwrap();
            all_date.push(date.to_string());
        }
    }

    let mut res = HashMap::new();

    for date_str in all_date {
        let dt: DateTime<Utc> = DateTime::parse_from_rfc3339(&date_str)
            .expect("Invalid date format")
            .with_timezone(&Utc);

        let iso_week: IsoWeek = dt.iso_week();
        let year = iso_week.year();
        let week = iso_week.week();

        let key = format!("{}-W{}", year, week);

        match res.get(&key) {
            Some(count) => res.insert(key, count + 1),
            None => res.insert(key, 1),
        };
    }

    res
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res = HashMap::new();

    if let JsonValue::Array(arr) = data {
        for item in arr.iter() {
            let name = item["author"]["login"].as_str().unwrap();
            match res.get(name) {
                Some(count) => res.insert(name.to_string(), count + 1),
                None => res.insert(name.to_string(), 1),
            };
        }
    }

    res
}
