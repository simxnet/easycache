use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Cache<K, V> {
    data: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new(ttl: Duration) -> Self {
        Cache {
            data: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, last_updated)) = self.data.get(key) {
            if last_updated.elapsed() < self.ttl {
                return Some(value);
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        let last_updated = Instant::now();
        self.data.insert(key, (value, last_updated));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some((value, _)) = self.data.remove(key) {
            return Some(value);
        }
        None
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}