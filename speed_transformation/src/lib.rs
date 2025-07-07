pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    // km/h -> m/s (10^3/60^2)
    return km_h * (1000. / (60. * 60.));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let km_h = 100.0;
        let m_s = km_per_hour_to_meters_per_second(km_h);
        assert_eq!(m_s, 27.77777777777778);
        println!("{} km/h is equivalent to {} m/s", km_h, m_s);
    }
}
