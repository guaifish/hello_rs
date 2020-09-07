#[macro_use]
extern crate derive_new;

#[derive(new, Debug)]
struct Foo {
    x: bool,
    #[new(value = "42")]
    y: i32,
    #[new(default)]
    z: Vec<String>,
}

fn main() {
    let a = Foo::new(true);
    println!("{:?}", a);
}