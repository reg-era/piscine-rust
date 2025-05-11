use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut res: f64 = 0.0;
    list.iter().for_each(|nb| res = (*nb as f64) + res);
    res / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut new_vec = Vec::from(list);

    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..new_vec.len() - 1 {
            if new_vec[i] > new_vec[i + 1] {
                new_vec.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    new_vec[new_vec.len() / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut new_list: HashMap<i32, i32> = HashMap::new();

    for nb in list {
        match new_list.get(nb) {
            Some(c) => new_list.insert(*nb, *c + 1),
            None => new_list.insert(*nb, 1),
        };
    }

    let mut res: (i32, i32) = (0, 0);
    for (k, v) in new_list {
        if res.1 < v {
            res = (k, v);
        }
    }
    res.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [4, 7, 5, 2, 5, 1, 3];

        println!("mean {}", mean(&v));
        println!("median {}", median(&v));
        println!("mode {}", mode(&v));
    }
}
