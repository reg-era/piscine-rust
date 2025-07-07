pub fn number_logic(num: u32) -> bool {
    fn get_len(mut nb: u32) -> u32 {
        let mut count = 0;
        while nb > 0 {
            nb = (nb as i32 / 10) as u32;
            count += 1;
        }
        count
    }

    let len = get_len(num);
    let mut copy = num as i32;

    let mut splited: Vec<i32> = Vec::new();
    while copy > 0 {
        let last = copy % 10;
        splited.push(last);
        copy = copy /10;
    }

    let mut total = 0;
    splited.into_iter().map(|n| n.pow(len)).for_each(|n| total += n);

    total as u32 == num
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
