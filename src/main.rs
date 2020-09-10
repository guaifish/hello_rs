use generic_array::{typenum::U5, ArrayLength, GenericArray};

#[derive(Debug)]
struct Foo<N: ArrayLength<i32>> {
    data: GenericArray<i32, N>,
}

fn main() {
    let foo = Foo::<U5> {
        data: GenericArray::default(),
    };
    println!("{:?}", foo);
}
