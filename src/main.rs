fn main() {
    let a1 = - 1;
    let a2 = - &1;
    println!("{}", a1);
    println!("{}", a2);

    let a3 = 1 + 2;
    let a4 = 1 + &2;
    let a5 = &1 + 2;
    let a6 = &1 + &2;
    println!("{}", a3);
    println!("{}", a4);
    println!("{}", a5);
    println!("{}", a6);

    let mut a7 = 0;
    a7 += 1;
    println!("{}", a7);
    a7 += &1;
    println!("{}", a7);
}