pub fn reverse_it(v: i32) -> String {
    let signed = v < 0;

    let itoa = if signed {
        (v * -1).to_string()
    } else {
        v.to_string()
    };
    let mut reversed = String::new();

    for i in (0..itoa.len()).rev() {
        reversed.push(itoa.chars().nth(i).unwrap());
    }

    let res = String::from(reversed + &itoa);
    if signed {
        "-".to_owned() + &res
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", reverse_it(123456789));
        println!("{}", reverse_it(-123));
    }

    #[test]
    fn reverse_it_test() {
        assert_eq!("321123", &reverse_it(123));
        assert_eq!("987654321123456789", &reverse_it(123456789));
        assert_eq!("00", &reverse_it(0));
        assert_eq!("-321123", &reverse_it(-123));
        assert_eq!("11", &reverse_it(1));
    }
}
