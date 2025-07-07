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

pub fn do_operations(v: &mut [String]) {
    for c in v {
        let wanted_op = if c.contains("+") { "+" } else { "-" };

        let index: usize = c.find(wanted_op).expect("wiiiw");
        let (part1, part2) = c.split_at(index);

        let num1: i32 = part1.parse().expect("wiiw");
        let num2: i32 = part2.parse().expect("wiiw");

        *c = (num1 + num2).to_string();
    }
}
