#[macro_use] extern crate prettytable;

fn main() {
    let table = table!(["ABC", "DEFG", "HIJKLMN"],
                       ["foobar", "bar", "foo"],
                       ["foobar2", "bar2", "foo2"]);

    table.printstd();
}