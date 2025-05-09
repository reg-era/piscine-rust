pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut nbrs: Vec<String> = Vec::new();
    for nb in a.split(" ") {
        let newnb: i32 = nb.parse().expect("wiiw");

        nbrs.push((newnb.abs() as f64).exp().to_string());
    }
    (a, nbrs.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res: Vec<f64> = Vec::new();
    for nbr in b.clone().into_iter() {
        res.push((nbr.abs() as f64).ln());
    }
    (b, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4, 5];
        let c = 0;

        println!("{:?}", nbr_function(c));
        println!("{:?}", vec_function(b));
        println!("{:?}", str_function(a));
    }
}
