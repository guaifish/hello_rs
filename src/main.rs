use std::ops::Add;
use typenum::{Integer, P3, P4};

fn main() {
    type X = <P3 as Add<P4>>::Output;
    assert_eq!(<X as Integer>::to_i32(), 7);
}
