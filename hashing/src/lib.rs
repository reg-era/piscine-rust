use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().map(|&x| x as f64).sum::<f64>() / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut newlist: Vec<i32> = Vec::from(list);
    newlist.sort();
    if newlist.len() % 2 == 0 && newlist.len() > 1 {
        let p1 = newlist[(newlist.len() / 2) - 1];
        let p2 = newlist[newlist.len() / 2];
        (p1 + p2) / 2
    } else {
        newlist[newlist.len() / 2]
    }
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
