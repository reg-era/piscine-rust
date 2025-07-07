pub fn negative_spell(n: i64) -> String {
    match n {
        1.. => "error: positive number".to_string(),
        0 => "zero".to_string(),
        _ => format!("minus {}", spell(-n as u64)),
    }
}

pub fn spell(n: u64) -> String {
    match n {
        0..=99 => spell_below_100(n),
        100..=999 => spell_hundreds(n),
        _ => spell_large(n),
    }
}

fn spell_below_100(n: u64) -> String {
    let under_20 = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    match n {
        0..=19 => under_20[n as usize].to_string(),
        20..=99 => {
            let t = n / 10;
            let u = n % 10;
            if u == 0 {
                tens[t as usize].to_string()
            } else {
                format!("{}-{}", tens[t as usize], under_20[u as usize])
            }
        }
        _ => unreachable!(),
    }
}

fn spell_hundreds(n: u64) -> String {
    let hundreds = n / 100;
    let remainder = n % 100;
    if remainder == 0 {
        format!("{} hundred", spell_below_100(hundreds))
    } else {
        format!(
            "{} hundred {}",
            spell_below_100(hundreds),
            spell_below_100(remainder)
        )
    }
}

fn spell_large(mut n: u64) -> String {
    let scales = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    let mut parts = vec![];
    let mut idx = 0;

    while n > 0 {
        let chunk = n % 1000;
        if chunk > 0 {
            let mut chunk_str = spell(chunk);
            if !scales[idx].is_empty() {
                chunk_str.push_str(&format!(" {}", scales[idx]));
            }
            parts.push(chunk_str);
        }
        n /= 1000;
        idx += 1;
    }

    parts.reverse();
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_numbers() {
        assert_eq!(negative_spell(0), String::from("zero"));
        assert_eq!(negative_spell(-1), String::from("minus one"));
        assert_eq!(negative_spell(-14), String::from("minus fourteen"));
        assert_eq!(negative_spell(-20), String::from("minus twenty"));
        assert_eq!(negative_spell(-22), String::from("minus twenty-two"));
        assert_eq!(negative_spell(-101), String::from("minus one hundred one"));
        assert_eq!(
            negative_spell(-120),
            String::from("minus one hundred twenty")
        );
        assert_eq!(
            negative_spell(-123),
            String::from("minus one hundred twenty-three")
        );
    }
    #[test]
    fn test_medium_numbers() {
        assert_eq!(negative_spell(-1000), String::from("minus one thousand"));
        assert_eq!(
            negative_spell(-1055),
            String::from("minus one thousand fifty-five")
        );
        assert_eq!(
            negative_spell(-1234),
            String::from("minus one thousand two hundred thirty-four")
        );
        assert_eq!(
            negative_spell(-10123),
            String::from("minus ten thousand one hundred twenty-three")
        );
    }
    #[test]
    fn test_long_numbers() {
        assert_eq!(
            negative_spell(-910112),
            String::from("minus nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            negative_spell(-651123),
            String::from("minus six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(
            negative_spell(-810000),
            String::from("minus eight hundred ten thousand")
        );
        assert_eq!(negative_spell(-1000000), String::from("minus one million"));
    }
    #[test]
    fn test_invalid_numbers() {
        assert_eq!(negative_spell(1), String::from("error: positive number"));
        assert_eq!(
            negative_spell(2390904),
            String::from("error: positive number")
        );
    }
}
