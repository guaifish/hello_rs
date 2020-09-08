#![feature(option_result_contains)]

fn main() {
    let mut x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    assert_eq!(x.contains(&2), true);
    assert_eq!(x.as_ref(), Some(&2));
    if let Some(v) = x.as_mut() {
        *v = 42;
    }
    assert_eq!(x, Some(42));
    x = Some(11);
    assert_eq!(x, Some(11));

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}