//! voca_rs::strip testing

#[test]
fn strip_bom() {
    assert_eq!(voca_rs::strip::strip_bom(""), "");
    assert_eq!(voca_rs::strip::strip_bom("\u{FEFF}"), "");
    assert_eq!(
        voca_rs::strip::strip_bom(voca_rs::utils::PRINTABLE),
        voca_rs::utils::PRINTABLE
    );
    assert_eq!(
        voca_rs::strip::strip_bom("\u{FEFF}summertime sadness"),
        "summertime sadness"
    );
    assert_eq!(
        voca_rs::strip::strip_bom("\\u{FEFF}summertime sadness"),
        "\\u{FEFF}summertime sadness"
    );
    assert_eq!(voca_rs::strip::strip_bom("summertime sadness"), "summertime sadness");
}
#[test]
fn strip_tags() {
    assert_eq!(voca_rs::strip::strip_tags(""), "");
    assert_eq!(
        voca_rs::strip::strip_tags("<span><a href=\"#\">Summer</a> is nice</span>"),
        "Summer is nice"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<html><b>hello</b><p>world</p></html>"),
        "helloworld"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("hello <img title='>_<'> world"),
        "hello  world"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<span class=\"italic\"><b>He>llo</b> < world!</span>"),
        "He>llo < world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<span class=\"<italic>\">Hello world!</span>"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<...<span class=\"<italic>\">Hello world!</span>"),
        "<...Hello world!"
    );
    assert_eq!(voca_rs::strip::strip_tags("< html >"), "< html >");
}