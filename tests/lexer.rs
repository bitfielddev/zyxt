#![feature(assert_matches)]
use std::assert_matches::assert_matches;

use pretty_assertions::assert_eq;
use proptest::prelude::*;
use smol_str::SmolStr;
use zyxt::types::token::TokenType;

macro_rules! lex {
    ($str:expr) => {
        zyxt::lexer::lex($str.to_owned(), "").unwrap()
    };
}

fn word_inner(s: String) {
    let re = lex!(s);
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_matches!(
        re[0].ty.unwrap(),
        TokenType::Ident | TokenType::Keyword(_) | TokenType::Flag(_)
    );
}
fn literal_int_inner(n: u128) {
    let re = lex!(n.to_string());
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(n.to_string()));
    assert_eq!(re[0].ty, Some(TokenType::LiteralNumber))
}
fn literal_float_inner(n: f64) {
    let re = lex!(n.to_string());
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(n.to_string()));
    assert_eq!(re[0].ty, Some(TokenType::LiteralNumber))
}
fn literal_string_inner(s: String) {
    let s = format!("\"{s}\"");
    let re = lex!(s);
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_eq!(re[0].ty, Some(TokenType::LiteralString))
}
fn line_comment_inner(s: String) {
    let s = format!("//{s}\n");
    let re = lex!(s);
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_eq!(re[0].ty, Some(TokenType::Comment))
}
fn block_comment_inner(s: String) {
    let s = format!("/*{s}*/");
    let re = lex!(s);
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_eq!(re[0].ty, Some(TokenType::Comment))
}
fn nested_block_comment_inner(s1: String, s2: String, s3: String) {
    let s = format!("/*{s1}/*{s2}*/{s3}*/");
    let re = lex!(s);
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(s));
    assert_eq!(re[0].ty, Some(TokenType::Comment))
}
fn whitespace_inner(s: String) {
    let s2 = format!("a{s}a");
    let re = lex!(s2);
    assert_eq!(re.len(), 2);
    assert_eq!(re[1].whitespace, SmolStr::from(s));
}
#[test]
fn symbol() {
    let re = lex!(":=");
    assert_eq!(re.len(), 1);
    assert_eq!(re[0].value, SmolStr::from(":="));
    assert_eq!(re[0].ty, Some(TokenType::DeclarationOpr));
}
proptest! {
    #[test]
    fn word(s in "[A-Za-z_][0-9A-Za-z_]{1,}".prop_filter("", |s| !["true", "false"].contains(&&**s))) {
        word_inner(s)
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
    fn literal_string(s in any::<String>().prop_filter("", |s| !s.contains('"'))) {
        literal_string_inner(s)
    }
    #[test]
    fn line_comment(s in any::<String>().prop_filter("", |s| !s.contains('\n'))) {
        line_comment_inner(s)
    }
    #[test]
    fn block_comment(s in any::<String>().prop_filter("", |s| !s.contains("/*") && !s.contains("*/") && !s.ends_with('/'))) {
        block_comment_inner(s)
    }
    #[test]
    fn nested_block_comment(
        s1 in any::<String>().prop_filter("", |s| !s.contains("/*") && !s.contains("*/") && !s.ends_with('*')),
        s2 in any::<String>().prop_filter("", |s| !s.contains("/*") && !s.contains("*/") && !s.ends_with('/')),
        s3 in any::<String>().prop_filter("", |s| !s.contains("/*") && !s.contains("*/") && !s.ends_with('/'))
    ) {
        nested_block_comment_inner(s1, s2, s3)
    }
    #[test]
    fn whitespace(s in r"\s+") {
        whitespace_inner(s)
    }
}
