fn main() {
    let a: u8 = 0b10001010;
    let b = reverse_bits(a);
    println!("{:b}", b);
}
fn reverse_bits(byte: u8) -> u8 {
    let mut result = 0;
    for i in 0..8 {
        result |= ((byte >> i) & 1) << (8 - 1 - i);
    }
    result
}
