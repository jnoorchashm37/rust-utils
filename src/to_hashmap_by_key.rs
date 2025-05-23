use std::{collections::HashMap, hash::Hash};

pub trait ToHashMapByKey: Iterator {
    fn hashmap_by_key<K: Eq + Hash>(
        self,
        f: impl Fn(&<Self as Iterator>::Item) -> K,
    ) -> HashMap<K, <Self as Iterator>::Item>;

    fn hashmap_by_key_val<K: Eq + Hash, V>(
        self,
        f: impl Fn(<Self as Iterator>::Item) -> (K, V),
    ) -> HashMap<K, V>;
}

impl<I: Iterator> ToHashMapByKey for I {
    fn hashmap_by_key<K: Eq + Hash>(
        self,
        f: impl Fn(&<Self as Iterator>::Item) -> K,
    ) -> HashMap<K, <Self as Iterator>::Item> {
        self.map(|v| (f(&v), v)).collect()
    }

    fn hashmap_by_key_val<K: Eq + Hash, V>(
        self,
        f: impl Fn(<Self as Iterator>::Item) -> (K, V),
    ) -> HashMap<K, V> {
        self.map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_by_key() {
        let vals = vec![2, 4, 6];

        let expected = HashMap::from_iter(vec![(1, 2), (2, 4), (3, 6)]);
        let calculated = vals.into_iter().hashmap_by_key(|v| *v / 2);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_hashmap_by_key_val() {
        let vals = vec![2, 4, 6];

        let expected = HashMap::from_iter(vec![(1, 4), (2, 8), (3, 12)]);
        let calculated = vals.into_iter().hashmap_by_key_val(|v| (v / 2, v * 2));

        assert_eq!(expected, calculated);
    }
}
