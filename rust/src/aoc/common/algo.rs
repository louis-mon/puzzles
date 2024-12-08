use std::collections::HashMap;
use std::hash::Hash;

pub fn group_by_key<K: Eq + Hash, V, I>(it: I) -> HashMap<K, Vec<V>>
where
    I: Iterator<Item = (K, V)>,
{
    let mut m: HashMap<K, Vec<V>> = HashMap::new();
    for (k, v) in it {
        m.entry(k).or_default().push(v);
    }
    m
}
