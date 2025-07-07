pub fn num_to_ordinal(x: u32) -> String {
    let card = if x % 10 == 1 {
        "st"
    } else if x % 10 == 2 && x > 12 {
        "nd"
    } else if x % 10 == 3 && x > 13 {
        "rd"
    } else if x == 1 && x > 11 {
        "st"
    } else {
        "th"
    };

    String::from(format!("{}{}", x, card))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", num_to_ordinal(1));
        println!("{}", num_to_ordinal(22));
        println!("{}", num_to_ordinal(43));
        println!("{}", num_to_ordinal(47));
    }
}
