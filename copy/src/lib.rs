pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let nbrs: Vec<String> = a.split(" ").map(|nb| {
        let newnb: i32 = nb.parse().expect("wiiw");
        (newnb.abs() as f64).exp().to_string()
    }).collect();

    (a, nbrs.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (b.clone(), b.into_iter().map(|nbr| (nbr.abs() as f64).ln() ).collect())
}
