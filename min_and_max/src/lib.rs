pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let best_min: i32 = nb_1.min(nb_2);
    let best_max: i32 = nb_1.max(nb_2);

    let min: i32 = best_min.min(nb_3);
    let max: i32 = best_max.max(nb_3);

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Minimum and maximum are: {:?}", min_and_max(9, 2, 4));
    }

    #[test]
    fn test_min_and_max() {
        let (min, max) = min_and_max(0, 0, 0);

        assert_eq!(min, 0);
        assert_eq!(max, 0);

        let (min, max) = min_and_max(1, 2, 3);

        assert_eq!(min, 1);
        assert_eq!(max, 3);

        let (min, max) = min_and_max(-1, -2, -3);

        assert_eq!(min, -3);
        assert_eq!(max, -1);

        let (min, max) = min_and_max(14, -12, 3);

        assert_eq!(min, -12);
        assert_eq!(max, 14);
    }
}
