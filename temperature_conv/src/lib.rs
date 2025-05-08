pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.) / 1.8;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (9. / 5. * c) + 32.;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fahrenheit_to_celsius(-459.67), -273.15);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);

        assert_eq!(fahrenheit_to_celsius(20.), -6.666666666666666);
        assert_eq!(fahrenheit_to_celsius(83.), 28.333333333333332);

        assert_eq!(celsius_to_fahrenheit(27.), 80.6);
    }
}
