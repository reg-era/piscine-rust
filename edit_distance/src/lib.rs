pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut levensh: Vec<Vec<usize>> = vec![vec![0; target.len() + 1]; source.len() + 1];

    for i in 0..=source.len() {
        levensh[i][0] = i;
    }
    for j in 0..=target.len() {
        levensh[0][j] = j;
    }

    for (i, sc) in source.chars().enumerate() {
        for (j, tc) in target.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };

            levensh[i + 1][j + 1] = (levensh[i][j] + cost)
                .min(levensh[i + 1][j] + 1)
                .min(levensh[i][j + 1] + 1);
        }
    }

    // for roe in levensh.iter() {
    // println!("{:?}",roe);
    // }

    levensh[source.len()][target.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source = "alignment";
        let target = "assignment";

        println!(
            "It's necessary to make {} change(s) to {:?} to get {:?}",
            edit_distance(source, target),
            source,
            target
        );
    }
}
