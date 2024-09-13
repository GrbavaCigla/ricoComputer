use ricocomputer::{parsers::declaration::decl, types::Declaration};

#[test]
fn declaration_with_spaces() {
    let dec = Declaration {symbol: String::from("s1M_bol"), value: 5};

    assert_eq!(decl("s1M_bol = 5").unwrap().1, dec);
    assert_eq!(decl("s1M_bol   = 5").unwrap().1, dec);
    assert_eq!(decl("s1M_bol   =    5").unwrap().1, dec);
}

#[test]
fn declaration_without_spaces() {
    let dec = Declaration {symbol: String::from("s1M_bol"), value: 5};

    assert_eq!(decl("s1M_bol=5").unwrap().1, dec);
}