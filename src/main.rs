fn main() {
    use chrono::prelude::*;

    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("{}", utc);
    println!("{}", local);
}
