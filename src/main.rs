use std::collections::HashMap;
use lru::LruCache;

fn main() {
    let mut m = HashMap::new();
    m.insert("apple", 3);
    m.insert("banana", 2);
    assert_eq!(m.get(&"apple"), Some(&3));
    assert_eq!(m.get("banana"), Some(&2));
    // pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    // where
    //     K: Borrow<Q>,
    //     Q: Hash + Eq,

    let mut cache = LruCache::new(2);
    cache.put("apple", 3);
    cache.put("banana", 2);
    assert_eq!(cache.get(&"apple"), Some(&3));
    // error
    assert_eq!(cache.get("banana"), Some(&2));
    // pub fn get<'a, Q>(&'a mut self, k: &Q) -> Option<&'a V>
    // where
    //     KeyRef<K>: Borrow<Q>,
    //     Q: Hash + Eq + ?Sized,
    // 
    // pub struct KeyRef<K> {
    //     k: *const K,
    // }
}