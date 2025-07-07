pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' => (((c as u8 - b'A' + 21) % 26) + b'A') as char,
            'a'..='z' => (((c as u8 - b'a' + 21) % 26) + b'a') as char,
            _ => c, // Non-alphabetic characters stay the same
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rot21_multiple_cases() {
        assert_eq!("ocdn dn v ozno", rot21("this is a test"));
        assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
        assert_eq!(
            "ojj  hpxc    nkvxzn      rjmfn",
            rot21("too  much    spaces      works")
        );
        assert_eq!("mv😋w", rot21("ra😋b"));
        assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

        assert_eq!("VWXY", rot21("ABCD"));
        assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
        assert_eq!("WTZ", rot21("BYE"));
    }
}
