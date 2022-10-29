use pretty_assertions::assert_eq;
use zyxt::types::{
    element::{
        binary_opr::BinaryOpr, declare::Declare, ident::Ident, set::Set, Element, ElementVariant,
    },
    position::{PosRaw, Position},
    token::{Flag, OprType},
};

macro_rules! parse {
    ($str:expr) => {
        zyxt::parser::parse_token_list(zyxt::lexer::lex($str.to_owned(), "").unwrap()).unwrap()
    };
}
macro_rules! pos_raw {
    ($line:literal, $column:literal, $raw:expr) => {
        PosRaw {
            pos: Position {
                filename: "".into(),
                line: $line,
                column: $column,
            },
            raw: $raw.into(),
        }
    };
}

#[test]
fn assignment() {
    let ast = parse!("x = foo");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x = foo"),
            data: Box::new(ElementVariant::Set(Set {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 5, " foo"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "foo".into(),
                        parent: None
                    }))
                }
            }))
        }
    )
}

#[test]
fn assignment_bin() {
    let ast = parse!("x += foo");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x += foo"),
            data: Box::new(ElementVariant::Set(Set {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " foo"),
                    data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                        ty: OprType::Plus,
                        operand1: Element {
                            pos_raw: pos_raw!(1, 1, "x"),
                            data: Box::new(ElementVariant::Ident(Ident {
                                name: "x".into(),
                                parent: None
                            })),
                        },
                        operand2: Element {
                            pos_raw: pos_raw!(1, 6, " foo"),
                            data: Box::new(ElementVariant::Ident(Ident {
                                name: "foo".into(),
                                parent: None
                            })),
                        }
                    })),
                }
            }))
        }
    )
}

#[test]
fn declaration() {
    let ast = parse!("x := foo");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x := foo"),
            data: Box::new(ElementVariant::Declare(Declare {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " foo"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "foo".into(),
                        parent: None
                    }))
                },
                flags: vec![],
                ty: None
            }))
        }
    )
}
#[test]
fn declaration_flags() {
    let ast = parse!("pub x := foo");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "pub x := foo"),
            data: Box::new(ElementVariant::Declare(Declare {
                variable: Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 10, " foo"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "foo".into(),
                        parent: None
                    }))
                },
                flags: vec![Flag::Pub],
                ty: None
            }))
        }
    )
}
