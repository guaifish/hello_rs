use http::uri::Scheme;
use http::Uri;

fn main() {
    let uri = "https://www.rust-lang.org/index.html"
        .parse::<Uri>()
        .unwrap();

    assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
    assert_eq!(uri.host(), Some("www.rust-lang.org"));
    assert_eq!(uri.path(), "/index.html");
    assert_eq!(uri.query(), None);
}
