#[allow(dead_code)]
#[derive(Default, Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}


#[allow(dead_code)]
#[derive(Debug)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::A }
}


fn main() {
    let options: SomeOptions = Default::default();
    println!("{:?}", options);
    let a: Kind = Default::default();
    println!("{:?}", a);
}