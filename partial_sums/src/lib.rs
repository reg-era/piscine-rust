pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    let mut index = arr.len();
    while res.len() < arr.len() {
        let mut count: u64 = 0;
        for nb in 0..index {
            count += arr[nb];
        }
        index -= 1;
        res.push(count);
    }
    res.push(0);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "Partial sums of [5, 18, 3, 23] is : {:?}",
            parts_sums(&[5, 18, 3, 23])
        );
    }

    #[test]
    fn simple_tests() {
        assert_eq!(vec![15, 10, 6, 3, 1, 0], parts_sums(&[1, 2, 3, 4, 5]));
        assert_eq!(vec![49, 26, 23, 5, 0], parts_sums(&[5, 18, 3, 23]));
        assert_eq!(vec![0, 0, 0, 0, 0, 0], parts_sums(&[0, 0, 0, 0, 0]))
    }

    #[test]
    fn complex_tests() {
        assert_eq!(
            vec![72393, 33352, 33350, 32787, 22597, 22515, 0],
            parts_sums(&[22515, 82, 10190, 563, 2, 39041])
        );
        assert_eq!(
            vec![
                2337, 2333, 2240, 2208, 2113, 2016, 2001, 1987, 1938, 1936, 1929, 1911, 1811, 1780,
                1729, 1728, 1653, 1593, 1521, 1459, 1369, 1273, 1197, 1155, 1107, 1084, 1017, 995,
                927, 918, 878, 814, 722, 693, 609, 581, 568, 477, 383, 344, 339, 262, 193, 113, 93,
                81, 0
            ],
            parts_sums(&[
                81, 12, 20, 80, 69, 77, 5, 39, 94, 91, 13, 28, 84, 29, 92, 64, 40, 9, 68, 22, 67,
                23, 48, 42, 76, 96, 90, 62, 72, 60, 75, 1, 51, 31, 100, 18, 7, 2, 49, 14, 15, 97,
                95, 32, 93, 4
            ])
        )
    }
}
