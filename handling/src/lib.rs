use std::fs;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let getting_file = fs::read(path);

    match getting_file {
        Ok(mut data) => {
            data.extend_from_slice(content.as_bytes());
            if let Err(_) = fs::write(path, data) {
                panic!("wiiiw error");
            }
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                if let Err(_) = fs::write(path, content) {
                    panic!("wiiiw error");
                }
            }
            _ => panic!("wiiiw error"),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = "a.txt";

        open_or_create(&path, "content to be written");

        let contents = fs::read_to_string(path).unwrap();
        println!("{}", contents);
    }
}
