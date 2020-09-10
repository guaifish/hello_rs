use fnv::FnvHashMap;

fn main() {
    let mut map = FnvHashMap::default();
    map.insert(1, "one");
    map.insert(2, "two");

    map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
    map.insert(1, "one");
    map.insert(2, "two");
    println!("{:?}", map);
}
