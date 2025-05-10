pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut linked = String::new();
    let mut count = 0;

    let mut index = 0;
    while linked != target {
        if index > source.len() {
            break;
        }
        if source.chars().nth(index) != target.chars().nth(index) {
            count += 1;
        } else {
            linked.push(target.chars().nth(index).expect("wiiw"));
        }
        index += 1;
    }
    if index != source.len() {
        count += target.len() - index;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source = "alignment";
        let target = "assignment";
        
        println!(
        "It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
        );
        println!("{} f {}", edit_distance("gumbo", "gambol"), 2);
        println!("{} f {}", edit_distance("kitten", "sitting"), 3);
        println!("{} f {}", edit_distance("rosettacode", "raisethysword"), 8);
    }
}
