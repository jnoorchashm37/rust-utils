use std::{collections::HashMap, hash::Hash};

pub trait ToHashMapByKey: Iterator {
    fn hashmap_by_key<K: Eq + Hash>(
        self,
        f: impl Fn(&<Self as Iterator>::Item) -> K,
    ) -> HashMap<K, <Self as Iterator>::Item>;
}

impl<I: Iterator> ToHashMapByKey for I {
    fn hashmap_by_key<K: Eq + Hash>(
        self,
        f: impl Fn(&<Self as Iterator>::Item) -> K,
    ) -> HashMap<K, <Self as Iterator>::Item> {
        self.into_iter().map(|v| (f(&v), v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let vals = vec![2, 4, 6];

        let calculated_iter = vals
            .iter()
            .hashmap_by_key(|v| **v / 2)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect::<HashMap<_, _>>();
        let calculated_into_iter = vals.into_iter().hashmap_by_key(|v| *v / 2).clone();

        let expected = HashMap::from_iter(vec![(1, 2), (2, 4), (3, 6)]);

        assert!(calculated_iter == calculated_into_iter);
        assert!(calculated_iter == expected);
        assert!(expected == calculated_into_iter);
    }
}
