fn main() {
    // let a = 3 ** 6;
    let a = 3_i32.pow(2);
    println!("{}", a);
    let a = 23_f64 / 7_f64;
    println!("{}", a);
    let _ = 1f64;
    // 字符串连接和复制
    let a = "Alice";
    let b = "Bob";
    let c = a.to_owned() + b;
    println!("{} {} {}", a, b, c);
    let d = a.repeat(3);
    println!("{}", d.len());
    let a = "123";
    let b = "3.14";
    let a: i32 = a.parse().unwrap();
    let b: f64 = b.parse().unwrap();
    assert_eq!(a, 123);
    assert_eq!(b, 3.14);
    assert_eq!(format!("{}", a), "123");
}
