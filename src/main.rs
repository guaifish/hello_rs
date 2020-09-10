#[macro_use]
extern crate maplit;

fn main() {
    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    println!("{:?}", map);

    let set = hashset!{"a", "b"};
    println!("{:?}", set);
}
