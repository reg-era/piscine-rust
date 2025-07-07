pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) / 1.8
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9. / 5. * c) + 32.
}