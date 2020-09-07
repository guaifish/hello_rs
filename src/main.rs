use displaydoc::Display;

/// p({x:?}, {y:?})
#[derive(Debug, Display)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point{ x: 1, y: 2 };
    println!("{}", p1);
}