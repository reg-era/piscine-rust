pub fn get_diamond(c: char) -> Vec<String> {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut res: Vec<String> = Vec::new();

    let mut count = 0;
    for x in alphabet.chars() {
        count += 1;
        if c == x {
            break;
        }
    }

    for (i, x) in alphabet.chars().enumerate() {
        let space = " ".repeat(count - i - 1);
        if i == 0 {
            res.push(format!("{}{}{}", space, x, space));
        } else {
            let midle_space = " ".repeat(i * 2 - 1);
            res.push(format!("{}{}{}{}{}", space, x, midle_space, x, space));
        }
        if x == c {
            break;
        }
    }

    let mut combined = res.clone();
    let mut rev = res.clone();
    rev.pop().unwrap();
    rev.reverse();
    combined.extend(rev);
    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", get_diamond('A'));
        println!("{:?}", get_diamond('C'));
        for line in get_diamond('C') {
            println!("{}", line);
        }
    }
}
