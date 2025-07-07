pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut res: Box<Vec<u32>> = Box::new(Vec::new());

    for mut lkalima in s.split(" ") {
        let mut multipl = 1.;
        if lkalima.ends_with("k") {
            lkalima = lkalima.strip_suffix("k").unwrap();
            multipl = 1000.;
        }

        let nb = lkalima.parse::<f64>().expect("wiiiw");
        res.push((nb * multipl) as u32);
    }

    res
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");

        // creating a variable and we save it in the Heap
        let a_h = transform_and_save_on_heap(new_str);
        println!("Box value : {:?}", &a_h);
        println!(
            "size occupied in the stack : {:?} bytes",
            (std::mem::size_of_val(&a_h))
        );

        let a_b_v = take_value_ownership(a_h);
        println!("value : {:?}", &a_b_v);
        println!(
            "size occupied in the stack : {:?} bytes",
            (std::mem::size_of_val(&a_b_v))
        );
        // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
    }
}
