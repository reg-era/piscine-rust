pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    // for c in dp.clone() {
        // println!("ss {:?}", c);
    // }

    for (i, sc) in source.chars().enumerate() {
        for (j, tc) in target.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };
            dp[i + 1][j + 1] = *[dp[i][j + 1] + 1, dp[i + 1][j] + 1, dp[i][j] + cost]
                .iter()
                .min()
                .unwrap();
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let source = "alignment";
        // let target = "assignment";
        //
        // println!(
        // "It's necessary to make {} change(s) to {:?} to get {:?}",
        // edit_distance(source, target),
        // source,
        // target
        // );
        println!("{} f {}", edit_distance("gumbo", "gambol"), 2);
        // println!("{} f {}", edit_distance("kitten", "sitting"), 3);
        // println!("{} f {}", edit_distance("rosettacode", "raisethysword"), 8);
    }
}
//    '  g  a  m  b  o  l
// ' [0, 1, 2, 3, 4, 5, 6]
// g [1, 0, 1, 2, 3, 4, 5]
// u [2, 1, 1, 2, 3, 4, 5]
// m [3, 2, 2, 1, 2, 3, 4]
// b [4, 3, 3, 2, 1, 2, 3]
// o [5, 4, 4, 3, 2, 1, 2]
