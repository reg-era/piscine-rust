use chrono::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 {
        return None;
    } else {
        let middle = NaiveDate::from_yo_opt(year as i32, f64::ceil(365.0 / 2.0) as u32);

        match middle {
            Some(date) => {
                let dt = Utc
                    .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
                    .unwrap();

                Some(dt.weekday())
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", middle_day(1022));
    }
}
