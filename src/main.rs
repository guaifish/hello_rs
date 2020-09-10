use const_random::const_random;

const MY_RANDOM_NUMBER: u32 = const_random!(u32);

fn main() {
    println!("{}", MY_RANDOM_NUMBER);
}
