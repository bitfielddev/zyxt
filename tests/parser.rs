use pretty_assertions::assert_eq;
use zyxt::types::{
    element::{
        binary_opr::BinaryOpr,
        block::Block,
        call::Call,
        declare::Declare,
        defer::Defer,
        delete::Delete,
        ident::Ident,
        literal::Literal,
        preprocess::Preprocess,
        procedure::Procedure,
        r#if::{Condition, If},
        r#return::Return,
        set::Set,
        unary_opr::UnaryOpr,
        Element, ElementVariant,
    },
    position::{PosRaw, Position},
    token::{Flag, OprType},
    typeobj::unit_t::UNIT_T,
    value::Value,
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
macro_rules! ident {
    ($name:expr) => {
        Box::new(ElementVariant::Ident(Ident {
            name: $name.into(),
            parent: None,
        }))
    };
    (notvar $name:expr) => {
        Box::new(Ident {
            name: $name.into(),
            parent: None,
        })
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
                    data: ident!("x")
                },
                content: Element {
                    pos_raw: pos_raw!(1, 5, " y"),
                    data: ident!("y")
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
                    data: ident!("x")
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " y"),
                    data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                        ty: OprType::Add,
                        operand1: Element {
                            pos_raw: pos_raw!(1, 1, "x"),
                            data: ident!("x")
                        },
                        operand2: Element {
                            pos_raw: pos_raw!(1, 6, " y"),
                            data: ident!("y"),
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
                        data: ident!("x")
                    },
                    operand2: Element {
                        pos_raw: pos_raw!(1, 4 + sy.len(), " y"),
                        data: ident!("y")
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
                    data: ident!("x")
                },
                content: Element {
                    pos_raw: pos_raw!(1, 6, " y"),
                    data: ident!("y")
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
                    data: ident!("x")
                },
                content: Element {
                    pos_raw: pos_raw!(1, 10, " y"),
                    data: ident!("y")
                },
                flags: vec![Flag::Pub],
                ty: None
            }))
        }
    )
}

#[test]
fn delete_single() {
    let ast = parse!("del x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "del x"),
            data: Box::new(ElementVariant::Delete(Delete {
                names: vec![Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: Box::new(Ident {
                        name: "x".into(),
                        parent: None
                    })
                }]
            }))
        }
    )
}

#[test]
fn delete_multiple() {
    let ast = parse!("del x, y, z");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "del x, y, z"),
            data: Box::new(ElementVariant::Delete(Delete {
                names: vec![
                    Element {
                        pos_raw: pos_raw!(1, 5, " x"),
                        data: ident!(notvar "x")
                    },
                    Element {
                        pos_raw: pos_raw!(1, 8, " y"),
                        data: ident!(notvar "y")
                    },
                    Element {
                        pos_raw: pos_raw!(1, 11, " z"),
                        data: ident!(notvar "z")
                    }
                ]
            }))
        }
    )
}

#[test]
#[ignore]
fn if_() {
    let ast = parse!("if x { }");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "if x { }"),
            data: Box::new(ElementVariant::If(If {
                conditions: vec![Condition {
                    condition: Some(Element {
                        pos_raw: pos_raw!(1, 4, " x"),
                        data: ident!("x")
                    }),
                    if_true: Element {
                        pos_raw: pos_raw!(1, 6, "{ }"),
                        data: Box::new(Block {
                            content: vec![Element {
                                pos_raw: pos_raw!(0, 0, ""),
                                data: Box::new(ElementVariant::Literal(Literal {
                                    content: Value::Unit
                                }))
                            }]
                        })
                    }
                }]
            }))
        }
    )
}

#[test]
#[ignore]
fn if_else() {}

#[test]
#[ignore]
fn if_elif() {}

#[test]
#[ignore]
fn if_elif_else() {}

#[test]
fn parentheses() {
    let ast = parse!("(x)");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 2, "(x)"),
            data: Box::new(ElementVariant::Ident(Ident {
                name: "x".into(),
                parent: None
            }))
        }
    )
}

#[test]
fn block() {
    let ast = parse!("{x}");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "{x}"),
            data: Box::new(ElementVariant::Block(Block {
                content: vec![Element {
                    pos_raw: pos_raw!(1, 2, "x"),
                    data: ident!("x")
                }]
            }))
        }
    )
}

#[test]
fn preprocess_block() {
    let ast = parse!("pre {x}");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "pre {x}"),
            data: Box::new(ElementVariant::Preprocess(Preprocess {
                content: Element {
                    pos_raw: pos_raw!(1, 5, " {x}"),
                    data: Box::new(ElementVariant::Block(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 6, "x"),
                            data: ident!("x")
                        }]
                    }))
                }
            }))
        }
    )
}

#[test]
fn preprocess_expr() {
    let ast = parse!("pre x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "pre x"),
            data: Box::new(ElementVariant::Preprocess(Preprocess {
                content: Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: ident!("x")
                }
            }))
        }
    )
}

#[test]
fn defer_block() {
    let ast = parse!("defer {x}");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "defer {x}"),
            data: Box::new(ElementVariant::Defer(Defer {
                content: Element {
                    pos_raw: pos_raw!(1, 7, " {x}"),
                    data: Box::new(ElementVariant::Block(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 8, "x"),
                            data: ident!("x")
                        }]
                    }))
                }
            }))
        }
    )
}

#[test]
fn defer_expr() {
    let ast = parse!("defer x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "defer x"),
            data: Box::new(ElementVariant::Defer(Defer {
                content: Element {
                    pos_raw: pos_raw!(1, 7, " x"),
                    data: ident!("x")
                }
            }))
        }
    )
}

#[test]
fn proc_kwd() {
    let ast = parse!("proc | | x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "proc | | x"),
            data: Box::new(ElementVariant::Procedure(Procedure {
                is_fn: false,
                args: vec![],
                return_type: None,
                content: Element {
                    pos_raw: pos_raw!(1, 10, " x"),
                    data: Box::new(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 10, " x"),
                            data: ident!("x")
                        }]
                    })
                }
            }))
        }
    )
}

#[test]
fn proc_nokwd() {
    let ast = parse!("| | x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "| | x"),
            data: Box::new(ElementVariant::Procedure(Procedure {
                is_fn: false,
                args: vec![],
                return_type: None,
                content: Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: Box::new(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 5, " x"),
                            data: ident!("x")
                        }]
                    })
                }
            }))
        }
    )
}

#[test]
fn fn_kwd() {
    let ast = parse!("fn | | x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "fn | | x"),
            data: Box::new(ElementVariant::Procedure(Procedure {
                is_fn: true,
                args: vec![],
                return_type: None,
                content: Element {
                    pos_raw: pos_raw!(1, 8, " x"),
                    data: Box::new(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 8, " x"),
                            data: ident!("x")
                        }]
                    })
                }
            }))
        }
    )
}

#[test]
fn fn_arg() {
    let ast = parse!("fn | | x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "fn | | x"),
            data: Box::new(ElementVariant::Procedure(Procedure {
                is_fn: true,
                args: vec![],
                return_type: None,
                content: Element {
                    pos_raw: pos_raw!(1, 8, " x"),
                    data: Box::new(Block {
                        content: vec![Element {
                            pos_raw: pos_raw!(1, 8, " x"),
                            data: ident!("x")
                        }]
                    })
                }
            }))
        }
    )
}

#[test]
fn return_nothing() {
    let ast = parse!("ret");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "ret"),
            data: Box::new(ElementVariant::Return(Return {
                value: UNIT_T.as_type().as_type_element().as_literal()
            }))
        }
    )
}

#[test]
fn return_something() {
    let ast = parse!("ret x");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "ret x"),
            data: Box::new(ElementVariant::Return(Return {
                value: Element {
                    pos_raw: pos_raw!(1, 5, " x"),
                    data: ident!("x")
                }
            }))
        }
    )
}

#[test]
#[ignore]
fn un_opr() {
    for (sy, ty) in [
        ("+", OprType::UnPlus),
        ("-", OprType::UnMinus),
        ("*", OprType::Deref),
        ("&", OprType::Ref),
        ("!", OprType::Not),
    ] {
        let s = format!("{sy}x");
        let ast = parse!(s);
        assert_eq!(
            ast[0],
            Element {
                pos_raw: pos_raw!(1, 1, s),
                data: Box::new(ElementVariant::UnaryOpr(UnaryOpr {
                    ty,
                    operand: Element {
                        pos_raw: pos_raw!(1, 2, "x"),
                        data: ident!("x")
                    },
                }))
            }
        )
    }
}

#[test]
fn unparen_call_single() {
    let ast = parse!("x y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x y"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                },
                args: vec![Element {
                    pos_raw: pos_raw!(1, 3, " y"),
                    data: ident!("y")
                }],
                kwargs: Default::default()
            }))
        }
    )
}

#[test]
fn unparen_call_multiple() {
    let ast = parse!("x y, z");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x y, z"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                },
                args: vec![
                    Element {
                        pos_raw: pos_raw!(1, 3, " y"),
                        data: ident!("y")
                    },
                    Element {
                        pos_raw: pos_raw!(1, 6, " z"),
                        data: ident!("z")
                    }
                ],
                kwargs: Default::default()
            }))
        }
    )
}

#[test]
fn unparen_call_nested() {
    let ast = parse!("x y z");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x y z"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                },
                args: vec![Element {
                    pos_raw: pos_raw!(1, 3, " y z"),
                    data: Box::new(ElementVariant::Call(Call {
                        called: Element {
                            pos_raw: pos_raw!(1, 3, " y"),
                            data: ident!("y")
                        },
                        args: vec![Element {
                            pos_raw: pos_raw!(1, 5, " z"),
                            data: ident!("z")
                        }],
                        kwargs: Default::default()
                    }))
                }],
                kwargs: Default::default()
            }))
        }
    )
}

#[test]
fn dot() {
    let ast = parse!("x.y");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 3, "y"),
            data: Box::new(ElementVariant::Ident(Ident {
                name: "y".into(),
                parent: Some(Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                })
            }))
        }
    )
}

#[test]
fn call_no_args() {
    let ast = parse!("x()");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x()"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                },
                args: vec![],
                kwargs: Default::default()
            }))
        }
    )
}

#[test]
fn call_with_args() {
    let ast = parse!("x(y)");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x(y)"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 1, "x"),
                    data: ident!("x")
                },
                args: vec![Element {
                    pos_raw: pos_raw!(1, 3, "y"),
                    data: ident!("y")
                }],
                kwargs: Default::default()
            }))
        }
    )
}

#[test]
fn dot_call() {
    let ast = parse!("x.y()");
    assert_eq!(
        ast[0],
        Element {
            pos_raw: pos_raw!(1, 1, "x.y()"),
            data: Box::new(ElementVariant::Call(Call {
                called: Element {
                    pos_raw: pos_raw!(1, 3, "y"),
                    data: Box::new(ElementVariant::Ident(Ident {
                        name: "y".into(),
                        parent: Some(Element {
                            pos_raw: pos_raw!(1, 1, "x"),
                            data: ident!("x")
                        })
                    }))
                },
                args: vec![],
                kwargs: Default::default()
            }))
        }
    )
}
