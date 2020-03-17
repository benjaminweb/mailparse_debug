extern crate mailparse;
use mailparse::*;
use std::fs::File;
use std::io::Read;

#[test]
fn test_html_without_attachment() {
    let mut file = File::open("without_att.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parsed = parse_mail(&contents.as_bytes()).unwrap();
    assert_eq!(parsed.subparts[0].get_body().unwrap(), "foo_text\r\n");
    assert_eq!(
        parsed.subparts[1].get_body().unwrap(),
        "<html>foo_html</html>"
    );
}

#[test]
fn test_html_with_attachment() {
    let mut file = File::open("with_att.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parsed = parse_mail(&contents.as_bytes()).unwrap();
    assert_eq!(parsed.subparts[0].get_body().unwrap(), "foo_text\r\n");
    assert_eq!(
        parsed.subparts[1].get_body().unwrap(),
        "<html>foo_html</html>"
    );
    assert_eq!(
        parsed.subparts[2].get_body().unwrap(),
        "r\n<<\n/Info 22 0 R\n/Root 1 0 R\n/Size 23\n>>\nstartxref\n427257\n%%EOF\n"
    );
}
