use std::fs::File;

pub fn open_file(s: &str) -> File {
    let getting_file = File::open(s);

    match getting_file {
        Ok(file) => file,
        Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {}",err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let filename = "created.txt";
        File::create(filename).unwrap();

        println!("{:?}", open_file(filename));

        std::fs::remove_file(filename).unwrap();

        // this should panic!
        open_file(filename);
    }
}
