#![feature(assert_matches)]

use std::assert_matches::assert_matches;

use proptest::prelude::*;
use smol_str::SmolStr;
use zyxt::{lexer::lex, types::token::TokenType};

fn word_inner(w: String) {
    let re = lex(w.to_owned(), "").unwrap();
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(w));
    assert_matches!(re[0].ty.unwrap(), TokenType::Ident | TokenType::Keyword(_));
}
fn literal_int_inner(n: u128) {
    let re = lex(n.to_string(), "").unwrap();
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(n.to_string()));
    assert_eq!(re[0].ty, Some(TokenType::LiteralNumber))
}
fn literal_float_inner(n: f64) {
    let re = lex(n.to_string(), "").unwrap();
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(n.to_string()));
    assert_eq!(re[0].ty, Some(TokenType::LiteralNumber))
}
fn literal_string_inner(s: String) {
    let s = format!("\"{s}\"");
    let re = lex(s.to_owned(), "").unwrap();
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_eq!(re[0].ty, Some(TokenType::LiteralString))
}
proptest! {
    #[test]
    fn word(w in "[A-Za-z_][0-9A-Za-z_]{1,}".prop_filter("", |w| !["true", "false"].contains(&&**w))) {
        word_inner(w)
    }
    #[test]
    fn literal_int(n in any::<u128>()) {
        literal_int_inner(n)
    }
    #[test]
    fn literal_float(n in any::<f64>().prop_filter("", |n| !n.is_nan() && !n.is_infinite())) {
        literal_float_inner(n.abs())
    }
    #[test]
    fn literal_string(w in "\\PC*".prop_filter("", |w| !w.contains('"'))) {
        literal_string_inner(w)
    }
}
