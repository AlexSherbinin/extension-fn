use std::{collections::HashMap, hash::Hash};

use extension_fn::extension_fn;


#[extension_fn(<K: Hash + Eq, V> HashMap<K, V>)]
pub fn merge(&mut self, other: Self) {
    for (key, value) in other.into_iter() {
        self.insert_if_not_exists(key, value);
    }
}

#[extension_fn(<K: Hash + Eq, V> HashMap<K, V>)]
fn insert_if_not_exists(&mut self, key: K, value: V) {
    if self.get(&key).is_none() {
        self.insert(key, value);
    }
}
