// X in the macro namespace
macro_rules! X {
    () => {};
}

// X in the type namespace
struct X {}

// X in the value namespace
const X: () = ();

fn main() {
    // unambiguously the macro X
    X!();

    // unambiguously the type X
    let _: X;

    // unambiguously the value X
    let _ = X;
}