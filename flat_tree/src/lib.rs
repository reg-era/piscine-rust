use std::collections::BTreeSet;

pub fn flatten_tree<T: ToOwned<Owned = T>>(tree: &BTreeSet<T>) -> Vec<T> {
    let mut res = Vec::new();
    for elem in tree {
        res.push(elem.to_owned());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BTreeSet::new();
        tree.insert(3);
        tree.insert(0);
        tree.insert(9);
        tree.insert(10);
        assert_eq!(flatten_tree(&tree), &[0, 3, 9, 10]);
    }

    #[test]
    fn test_with_str() {
        let mut tree = BTreeSet::new();
        tree.insert("Slow");
        tree.insert("kill");
        tree.insert("will");
        tree.insert("Horses");
        assert_eq!(flatten_tree(&tree), &["Horses", "Slow", "kill", "will"]);
    }
}