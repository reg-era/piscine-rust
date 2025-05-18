pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut res = String::new();

    let sliced: Vec<char> = message.chars().collect();
    for x in 0..i {
        if x < sliced.len() as u32 {
            res.push(sliced[x as usize]);
        }
        if x+i <= (sliced.len() - 1) as u32 {
            res.push(sliced[(x + i)as usize]);
        }else{
            res.push(' ');
        }
    }

    res.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "\"scytale Code\" size=6 -> {:?}",
            scytale_cipher(String::from("scytale Code"), 6)
        );
        println!(
            "\"scytale Code\" size=8 -> {:?}",
            scytale_cipher(String::from("scytale Code"), 8)
        );
    }
}
