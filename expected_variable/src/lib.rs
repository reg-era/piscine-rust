pub mod edit_distance;
pub use edit_distance::edit_distance;

pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    if str1.contains(|c| c == ' ' || c == '-') {
        return None;
    }

    let distance = edit_distance(
        &str1.to_lowercase().to_string(),
        &str2.to_lowercase().to_string(),
    );

    if distance > str1.len().max(str2.len()) / 2 {
        return None;
    }
    // println!("distance {}", distance);

    let percent: f64 = ((str2.len() - distance) as f64 / str2.len() as f64) * 100.0;

    // println!("percent {}", percent);
        Some(String::from(format!("{}%", percent.round())))
}

// test incorrect_cases ... FAILED
// test none_cases
