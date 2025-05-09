pub fn delete_and_backspace(s: &mut String) {
    let mut copy = String::new();
    let mut index = 0;
    for c in s.chars() {
        if c == '-' {
            copy.pop();
        } else if c == '+' {
            index += 1;
        } else if index > 0 {
            index -= 1;
        } else {
            copy.push(c);
        }
    }
    *s = copy;
}

// pub fn do_operations(v: &mut [String]) {
    // 
    // for c in s.chars() {
        // if c == '-' {
            // copy.pop();
        // } else if c == '+' {
            // index += 1;
        // } else if index > 0 {
            // index -= 1;
        // } else {
            // copy.push(c);
        // }
    // }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        // let mut b = [
            // "2+2".to_owned(),
            // "3+2".to_owned(),
            // "10-3".to_owned(),
            // "5+5".to_owned(),
        // ];

        delete_and_backspace(&mut a);
        // do_operations(&mut b);

        println!("{:?}", (a));
    }
}
