pub fn number_logic(num: u32) -> bool {
    num.to_string()
        .chars()
        .map(|c| {
            (c).to_string()
                .parse::<u32>()
                .unwrap()
                .pow(num.to_string().len() as u32)
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
        == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let array = [9, 10, 153, 154];
        for pat in &array {
            if number_logic(*pat) == true {
                println!(
                    "this number returns {} because the number {} obey the rules of the sequence",
                    number_logic(*pat),
                    pat
                )
            }
            if number_logic(*pat) == false {
                println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
            }
        }
    }
}
