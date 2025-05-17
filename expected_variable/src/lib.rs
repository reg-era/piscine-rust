pub mod edit_distance;
pub use edit_distance::edit_distance;

pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    let distance = edit_distance(
        &str1.to_lowercase().to_string(),
        &str2.to_lowercase().to_string(),
    );

    // println!("distance {}", distance);

    let percent: f64 = ((str2.len() - distance) as f64 / str2.len() as f64) * 100.0;

    // println!("percent {}", percent);
    if percent >= 50. {
        Some(String::from(format!("{}%", percent.round())))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{} close to it",
            expected_variable("On_Point", "on_point").unwrap()
        );
        println!(
            "{} close to it",
            expected_variable("soClose", "so_close").unwrap()
        );
        println!(
            "{:?}",
            expected_variable("something", "something_completely_different")
        );
        println!(
            "{} close to it",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}
