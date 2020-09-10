#[macro_use] extern crate derivative;

#[derive(Derivative)]
#[derivative(Debug)]
struct Foo {
    foo: u8,
    #[derivative(Debug = "ignore")]
    bar: u8,
}

fn main() {
    // Prints `Foo { foo: 42 }`
    println!("{:?}", Foo { foo: 42, bar: 1 });
}
