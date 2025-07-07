use std::collections::HashMap;

pub fn slices_to_map<'a, K: Eq + std::hash::Hash + Clone, V: Clone>(
    keys: &'a [K],
    values: &'a [V],
) -> HashMap<&'a K, &'a V> {
    let len = keys.len().min(values.len());
    let mut map = HashMap::with_capacity(len);

    for i in 0..len {
        map.insert(&keys[i], &values[i]);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_length() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        let mut expected = HashMap::new();
        expected.insert(&"Olivia", &1);
        expected.insert(&"Liam", &3);
        expected.insert(&"Emma", &23);
        expected.insert(&"Noah", &5);
        expected.insert(&"James", &2);
        assert_eq!(slices_to_map(&keys, &values), expected);
    }

    #[test]
    fn test_different_length() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2, 9];
        let mut expected = HashMap::new();
        expected.insert(&"Olivia", &1);
        expected.insert(&"Liam", &3);
        expected.insert(&"Emma", &23);
        expected.insert(&"Noah", &5);
        expected.insert(&"James", &2);
        assert_eq!(slices_to_map(&keys, &values), expected);

        let keys = ["Olivia", "Liam", "Emma", "Noah", "James", "Isabella"];
        let values = [1, 3, 23, 5, 2];
        assert_eq!(slices_to_map(&keys, &values), expected);
    }

    #[test]
    fn it_works_for_vecs() {
        let mut expected = HashMap::new();
        expected.insert(&"Olivia", &1);
        expected.insert(&"Liam", &3);
        expected.insert(&"Emma", &23);
        expected.insert(&"Noah", &5);
        expected.insert(&"James", &2);

        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];

        assert_eq!(slices_to_map(&keys, &values), expected);
        let keys = vec!["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = vec![1, 3, 23, 5, 2, 9];
        assert_eq!(slices_to_map(&keys, &values), expected);

        let keys = vec!["Olivia", "Liam", "Emma", "Noah", "James", "Isabella"];
        let values = vec![1, 3, 23, 5, 2];
        assert_eq!(slices_to_map(&keys, &values), expected);
    }
}
