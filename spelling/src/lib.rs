pub fn spell(n: u64) -> String {
    fn under_hundred(n: u64) -> String {
        match n {
            0 => "zero".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            20 => "twenty".to_string(),
            30 => "thirty".to_string(),
            40 => "forty".to_string(),
            50 => "fifty".to_string(),
            60 => "sixty".to_string(),
            70 => "seventy".to_string(),
            80 => "eighty".to_string(),
            90 => "ninety".to_string(),
            21..=99 => {
                let l3acharat = n / 10 * 10;
                let lwa7adat = n % 10;
                if lwa7adat == 0 {
                    under_hundred(l3acharat)
                } else {
                    format!("{}-{}", under_hundred(l3acharat), under_hundred(lwa7adat))
                }
            }
            _ => unreachable!(),
        }
    }

    fn under_thousand(n: u64) -> String {
        if n < 100 {
            return under_hundred(n);
        }

        let lmi2at = n / 100;
        let still = n % 100;

        if still == 0 {
            format!("{} hundred", under_hundred(lmi2at))
        } else {
            format!(
                "{} hundred {}",
                under_hundred(lmi2at),
                under_hundred(still)
            )
        }
    }

    if n == 0 {
        return "zero".to_string();
    }

    if n < 1_000 {
        return under_thousand(n);
    }

    let million = (n % 1_000_000_000) / 1_000_000;
    let thousand = (n % 1_000_000) / 1_000;
    let still = n % 1_000;

    let mut res = String::new();

    if million > 0 {
        res.push_str(&format!("{} million", under_thousand(million)));
    }

    if thousand > 0 {
        res.push_str(&format!("{} thousand", under_thousand(thousand)));
        if still > 0 {
            res.push_str(" ");
        }
    }

    if still > 0 {
        res.push_str(&under_thousand(still));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", spell(7));
        println!("{}", spell(96));
        println!("{}", spell(22));
        println!("{}", spell(30));
        println!("{}", spell(348));
        println!("{}", spell(999));
        println!("{}", spell(101));
        println!("{}", spell(1000));
        println!("{}", spell(1055));
    }
}
