pub fn delete_and_backspace(s: &mut String) {
    let mut saved = "";
    let mut index = 0;
    let mut move_up = false;
    for c in s.chars() {
        if c == '+' {
            move_up = true;
            index += 1;
        }
        if c == '-' {
            move_up = true;
            index -= 1;
        }
        if c != '-' && c != '+'  {
            if move_up && index > 0{
                move_up = false;
                *s[index] = 'd';
            }else{
                s.push(c);
            }
        }
    }
}

// pub fn do_operations(v: &mut [String]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];

        delete_and_backspace(&mut a);
        do_operations(&mut b);

        println!("{:?}", (a, b));
    }
}
