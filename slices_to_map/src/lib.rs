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
