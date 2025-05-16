#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mirore: Vec<u8> = Vec::from("zyxwvutsrqponmlkjihgfedcba");
    let mut main_ciphred = String::new();

    for c in original.chars() {
        if c.is_alphabetic() && c != ' ' {
            if c.is_uppercase() {
                main_ciphred.push_str(
                    &(mirore
                        [(c.to_lowercase().next().unwrap_or(c) as u8 as f64 - 97.0).abs() as usize]
                        as char)
                        .to_uppercase()
                        .to_string(),
                );
            } else {
                main_ciphred.push(mirore[(c as u8 as f64 - 97.0).abs() as usize] as char);
            }
        } else {
            main_ciphred.push(c);
        }
    }

    if main_ciphred == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: main_ciphred,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
        println!("{:?}", cipher("1Hello 2world!", "svool"));
    }
}
