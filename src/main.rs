use base_x::{encode, decode};

fn main() {
    let alphabet = "0123456789";
    let a = b"Hello, World!";
    let b = "5735816763073854918203775149089";
    assert_eq!(encode(alphabet, a), b);
    assert_eq!(a, &decode(alphabet, b).unwrap()[..]);
}