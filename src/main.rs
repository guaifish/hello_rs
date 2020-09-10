use cookie::Cookie;

fn main() {
    let cookie = Cookie::build("name", "value")
        .domain("www.rust-lang.org")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    println!("{:?}", cookie.to_string());
}
