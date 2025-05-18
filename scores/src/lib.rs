pub fn score(sc: &str) -> u64 {
    let mut acc = 0;
    let one = vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
    let two = vec!['D', 'G'];
    let three = vec!['B', 'C', 'M', 'P'];
    let four = vec!['F', 'H', 'V', 'W', 'Y'];
    let five = vec!['K'];
    let six = vec!['J', 'X'];
    let seven = vec!['Q', 'Z'];

    let to_make = String::from(sc.to_uppercase().to_string());
    for c in to_make.chars() {
        if one.contains(&c) {
            acc += 1
        };
        if two.contains(&c) {
            acc += 2
        };
        if three.contains(&c) {
            acc += 3
        };
        if four.contains(&c) {
            acc += 4
        };
        if five.contains(&c) {
            acc += 5
        };
        if six.contains(&c) {
            acc += 8
        };
        if seven.contains(&c) {
            acc += 10
        };
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", score("a"));
        println!("{}", score("ã ê Á?"));
        println!("{}", score("ThiS is A Test"));
    }
}
