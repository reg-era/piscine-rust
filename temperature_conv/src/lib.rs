pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // °C = (°F - 32) × 5/9
    return (f - 32.) * 5. / 9.;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // °F = (9/5 × °C) + 32
    return (9. / 5. * c ) + 32.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fahrenheit_to_celsius(-459.67), -273.15);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }
}
