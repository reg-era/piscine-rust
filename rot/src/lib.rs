pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let mut revc = c as i32 + key as i32;
            if c.is_ascii_lowercase() {
                if key < 0 {
                    revc += 26
                }
                if revc > 122 {
                    revc -= 26;
                }
                // println!("cc {} {}", char::from(revc as u8), revc);
                res.push(char::from(revc as u8));
            } else {
                if key < 0 {
                    revc += 26
                }
                if revc > 90 {
                    revc -= 26;
                }
                // println!("cc {} {}", char::from(revc as u8), revc);
                res.push(char::from(revc as u8));
            }
        } else {
            res.push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("The letter \"a\" becomes: {}", rotate("aza", 26));
        // println!("The letter \"m\" becomes: {}", rotate("m", 0));
        // println!("The letter \"m\" becomes: {}", rotate("m", 13));
        // println!("The letter \"a\" becomes: {}", rotate("a", 15));
        // println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
        // println!(
        // "The decoded message is: {}",
        // rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
        // );
        // println!(
        // "The decoded message is: {}",
        // rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
        // );
        println!(
            "Your cypher wil be: {}",
            rotate("Testing with numbers 1 2 3", 4)
        );
        println!("Your cypher wil be: {}", rotate("Testing", -14));
        println!("The letter \"a\" becomes: {}", rotate("a", -1));
    }
}
