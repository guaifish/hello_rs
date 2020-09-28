use bit_vec::BitVec;

fn main() {
    let mut bv = BitVec::from_elem(10, false);

    // insert all primes less than 10
    bv.set(2, true);
    bv.set(3, true);
    bv.set(5, true);
    bv.set(7, true);
    println!("{:?}", bv);
    println!(
        "total bits set to true: {}",
        bv.iter().filter(|x| *x).count()
    );

    // flip all values in bitvector, producing non-primes less than 10
    bv.negate();
    println!("{:?}", bv);
    println!(
        "total bits set to true: {}",
        bv.iter().filter(|x| *x).count()
    );

    // reset bitvector to empty
    bv.clear();
    println!("{:?}", bv);
    println!(
        "total bits set to true: {}",
        bv.iter().filter(|x| *x).count()
    );
}
