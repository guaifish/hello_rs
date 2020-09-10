extern crate deunicode;

use deunicode::deunicode;

fn main() {
    assert_eq!(deunicode("Æneid"), "AEneid");
    assert_eq!(deunicode("étude"), "etude");
    assert_eq!(deunicode("北亰"), "Bei Jing");
    assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
    assert_eq!(deunicode("げんまい茶"), "genmaiCha");
    assert_eq!(deunicode("🦄☣"), "unicorn biohazard");
    assert_eq!(deunicode("…"), "...");
    
    assert_eq!(deunicode("你好!"), "Ni Hao !");
    assert_eq!(deunicode("囧"), "Jiong");
    assert_eq!(deunicode("〇"), "0");
    assert_eq!(deunicode("零"), "Ling");
}
