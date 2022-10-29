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
    ($line:expr, $column:expr, $raw:expr) => {
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
    let ast = parse!("x = y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x = y"),
            data: Box::new(ElementVariant::Set(Set {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 5, " y"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "y".into(),
                        parent: None
                    }))
                }
            }))
        }
    )
}

#[test]
fn assignment_bin() {
    let ast = parse!("x += y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x += y"),
            data: Box::new(ElementVariant::Set(Set {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " y"),
                    data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                        ty: OprType::Add,
                        operand1: Element {
                            pos_raw: pos_raw!(1, 1, "x"),
                            data: Box::new(ElementVariant::Ident(Ident {
                                name: "x".into(),
                                parent: None
                            })),
                        },
                        operand2: Element {
                            pos_raw: pos_raw!(1, 6, " y"),
                            data: Box::new(ElementVariant::Ident(Ident {
                                name: "y".into(),
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
fn bin_opr() {
    for (sy, ty) in [
        ("+", OprType::Add),
        ("-", OprType::Sub),
        ("*", OprType::Mul),
        ("/", OprType::Div),
        ("^", OprType::Pow),
        ("%", OprType::Mod),
        ("~", OprType::Concat),
        ("@", OprType::TypeCast),
        ("==", OprType::Eq),
        ("!=", OprType::Ne),
        (">", OprType::Gt),
        (">=", OprType::Ge),
        ("<", OprType::Lt),
        ("<=", OprType::Le),
        ("&&", OprType::And),
        ("||", OprType::Or),
    ] {
        let s = format!("x {sy} y");
        let ast = parse!(s);
        assert_eq!(
            ast[0],
            Element {
                pos_raw: pos_raw!(1, 1, s),
                data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                    ty,
                    operand1: Element {
                        pos_raw: pos_raw!(1, 1, "x"),
                        data: Box::new(ElementVariant::Ident(Ident {
                            name: "x".into(),
                            parent: None
                        }))
                    },
                    operand2: Element {
                        pos_raw: pos_raw!(1, 4 + sy.len(), " y"),
                        data: Box::new(ElementVariant::Ident(Ident {
                            name: "y".into(),
                            parent: None
                        }))
                    }
                }))
            }
        )
    }
}

#[test]
#[ignore]
fn class_no_params() {
    let ast = parse!("class { }");
}

#[test]
#[ignore]
fn class_params() {
    let ast = parse!("class |x: i32| { }");
}

#[test]
#[ignore]
fn struct_content() {
    let ast = parse!("struct |x: i32| { }");
}

#[test]
#[ignore]
fn struct_no_content() {
    let ast = parse!("struct |x: i32|");
}

#[test]
#[ignore]
fn struct_no_params() {
    let ast = parse!("struct { }");
}

#[test]
#[ignore]
fn struct_no_content_no_params() {
    let ast = parse!("struct");
}

#[test]
fn declaration() {
    let ast = parse!("x := y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x := y"),
            data: Box::new(ElementVariant::Declare(Declare {
                variable: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " y"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "y".into(),
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
    let ast = parse!("pub x := y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "pub x := y"),
            data: Box::new(ElementVariant::Declare(Declare {
                variable: Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "x".into(),
                        parent: None
                    }))
                },
                content: Element {
                    pos_raw: pos_raw!(1, 10, " y"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "y".into(),
                        parent: None
                    }))
                },
                flags: vec![Flag::Pub],
                ty: None
            }))
        }
    )
}
