use std::collections::HashMap;
use std::hash::Hash;

pub trait GroupBy<K: Eq + Hash, V>: Iterator<Item = (K, V)> {
    fn group_by_key(self) -> HashMap<K, Vec<V>>;
}

impl<I, K: Eq + Hash, V> GroupBy<K, V> for I
where
    I: Iterator<Item = (K, V)>,
{
    fn group_by_key(self) -> HashMap<K, Vec<V>>
    where
        I: Iterator<Item = (K, V)>,
    {
        let mut m: HashMap<K, Vec<V>> = HashMap::new();
        for (k, v) in self {
            m.entry(k).or_default().push(v);
        }
        m
    }
}
