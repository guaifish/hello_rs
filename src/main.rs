#[macro_use]
extern crate derive_new;

use displaydoc::Display;

/// p({x:?}, {y:?})
#[derive(new, Display)]
struct Point {
    #[new(value = "1")]
    x: i32,
    #[new(default)]
    y: i32,
}

fn main() {
    let p = Point::new();
    println!("{}", p);
}