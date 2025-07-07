pub fn get_diamond(c: char) -> Vec<String> {
    let mut res = Vec::new();

    let n = c as u8 - b'A';
    for i in 0..=n {
        let letter = (b'A' + i) as char;
        let born_spc = (n - i) as usize;
        let midlle_spc = if i == 0 { 0 } else { (2 * i - 1) as usize };

        let line = if i == 0 {
            format!("{}{}{}", " ".repeat(born_spc), letter, " ".repeat(born_spc))
        } else {
            format!(
                "{}{}{}{}{}",
                " ".repeat(born_spc),
                letter,
                " ".repeat(midlle_spc),
                letter,
                " ".repeat(born_spc)
            )
        };

        res.push(line);
    }

    for i in (0..n).rev() {
        res.push(res[i as usize].clone());
    }

    res
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
