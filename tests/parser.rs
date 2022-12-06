use std::sync::Arc;

use once_cell::sync::Lazy;
use pretty_assertions::assert_eq;
use smol_str::SmolStr;
use zyxt::types::{
    element::{
        binary_opr::BinaryOpr,
        block::Block,
        call::Call,
        class::Class,
        declare::Declare,
        defer::Defer,
        delete::Delete,
        ident::Ident,
        literal::Literal,
        preprocess::Preprocess,
        procedure::{Argument, Procedure},
        r#if::{Condition, If},
        r#return::Return,
        set::Set,
        unary_opr::UnaryOpr,
        Element,
    },
    position::{Position, Span},
    token::{Flag, OprType},
    typeobj::unit_t::UNIT_T,
    value::Value,
};

static FILENAME_ARC: Lazy<Arc<SmolStr>> = Lazy::new(|| Arc::new("".into()));

macro_rules! parse {
    ($str:expr) => {
        zyxt::parser::parse_token_list(zyxt::lexer::lex($str.to_owned(), "".into()).unwrap())
            .unwrap()
    };
}
macro_rules! span {
    ($line:expr, $column:expr, $raw:expr) => {
        Span::new(
            Position {
                filename: Some(Arc::clone(&*FILENAME_ARC)),
                line: $line,
                column: $column,
            },
            $raw.into(),
        )
    };
}
macro_rules! ident {
    ($line:expr, $column:expr, $name:expr) => {
        Box::new(Element::Ident(Ident {
            name: $name.into(),
            parent: None,
            dot_span: None,
            name_span: Some(span!($line, $column, $name)),
        }))
    };
    (notvar $line:expr, $column:expr, $name:expr) => {
        Ident {
            name: $name.into(),
            parent: None,
            dot_span: None,
            name_span: Some(span!($line, $column, $name)),
        }
    };
}

#[test]
fn assignment() {
    let ast = parse!("x = y");
    assert_eq!(
        ast[0],
        Element::Set(Set {
            variable: ident!(1, 1, "x"),
            eq_span: Some(span!(1, 3, "=")),
            content: ident!(1, 5, "y")
        })
    )
}

#[test]
fn assignment_bin() {
    let ast = parse!("x += y");
    assert_eq!(
        ast[0],
        Element::Set(Set {
            variable: ident!(1, 1, "x").into(),
            eq_span: Some(span!(1, 3, "+=")),
            content: Element::BinaryOpr(BinaryOpr {
                ty: OprType::Add,
                opr_span: None,
                operand1: ident!(1, 1, "x").into(),
                operand2: ident!(1, 6, "y").into()
            })
            .into(),
        })
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
            Element::BinaryOpr(BinaryOpr {
                ty,
                opr_span: Some(span!(1, 3, sy)),
                operand1: ident!(1, 1, "x"),
                operand2: ident!(1, 4 + sy.len(), "y")
            })
        )
    }
}

#[test]
fn class() {
    let ast = parse!("class { }");
    assert_eq!(
        ast[0],
        Element::Class(Class {
            is_struct: false,
            implementations: Default::default(),
            inst_fields: Default::default(),
            content: Some(Block {
                brace_spans: None,
                content: vec![]
            }),
            args: None
        })
    )
}

#[test]
#[ignore]
fn struct_params() {
    let ast = parse!("struct |x: i32| { }");
    assert_eq!(
        ast[0],
        Element::Class(Class {
            is_struct: true,
            implementations: Default::default(),
            inst_fields: Default::default(),
            content: Some(Block {
                brace_spans: None,
                content: vec![]
            }),
            args: Some(vec![Argument {
                name: ident!(notvar 1, 1, "x"),
                ty: ident!(1, 11, "i32"),
                default: None
            }])
        })
    )
}

#[test]
#[ignore]
fn struct_no_content() {
    let ast = parse!("struct |x: i32|");
    assert_eq!(
        ast[0],
        Element::Class(Class {
            is_struct: true,
            implementations: Default::default(),
            inst_fields: Default::default(),
            content: None,
            args: Some(vec![Argument {
                name: ident!(notvar 1, 1, "x"),
                ty: ident!(1, 11, "i32").into(),
                default: None
            }])
        })
    )
}

#[test]
#[ignore]
fn struct_no_params() {
    let ast = parse!("struct { }");
    assert_eq!(
        ast[0],
        Element::Class(Class {
            is_struct: true,
            implementations: Default::default(),
            inst_fields: Default::default(),
            content: Some(Block {
                brace_spans: None,
                content: vec![]
            }),
            args: None
        })
    )
}

#[test]
#[ignore]
fn struct_no_content_no_params() {
    let ast = parse!("struct");
    assert_eq!(
        ast[0],
        Element::Class(Class {
            is_struct: true,
            implementations: Default::default(),
            inst_fields: Default::default(),
            content: None,
            args: None
        })
    )
}

#[test]
fn declaration() {
    let ast = parse!("x := y");
    assert_eq!(
        ast[0],
        Element::Declare(Declare {
            variable: ident!(1, 1, "x"),
            content: ident!(1, 6, "y"),
            flags: vec![],
            ty: None,
            eq_span: None,
        })
    )
}
#[test]
fn declaration_flags() {
    let ast = parse!("pub x := y");
    assert_eq!(
        ast[0],
        Element::Declare(Declare {
            variable: ident!(1, 5, "x"),
            content: ident!(1, 10, "y"),
            flags: vec![(Flag::Pub, span!(1, 1, "pub"))],
            ty: None,
            eq_span: None,
        })
    )
}

#[test]
fn delete_single() {
    let ast = parse!("del x");
    assert_eq!(
        ast[0],
        Element::Delete(Delete {
            kwd_span: Some(span!(1, 1, "del")),
            names: vec![ident!(notvar 1, 5, "x")]
        })
    )
}

#[test]
fn delete_multiple() {
    let ast = parse!("del x, y, z");
    assert_eq!(
        ast[0],
        Element::Delete(Delete {
            kwd_span: Some(span!(1, 1, "del")),
            names: vec![
                ident!(notvar 1, 5, "x"),
                ident!(notvar 1, 8, "y"),
                ident!(notvar 1, 11, "z")
            ]
        })
    )
}

#[test]
fn if_() {
    let ast = parse!("if x { }");
    assert_eq!(
        ast[0],
        Element::If(If {
            conditions: vec![Condition {
                kwd_span: None,
                condition: Some(*ident!(1, 4, "x")),
                if_true: Block {
                    brace_spans: None,
                    content: vec![]
                }
            }]
        })
    )
}

#[test]
fn if_else() {
    let ast = parse!("if x { } else { }");
    assert_eq!(
        ast[0],
        Element::If(If {
            conditions: vec![
                Condition {
                    kwd_span: None,
                    condition: Some(*ident!(1, 4, "x")),
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                },
                Condition {
                    kwd_span: None,
                    condition: None,
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                }
            ]
        })
    )
}

#[test]
fn if_elif() {
    let ast = parse!("if x { } elif y { }");
    assert_eq!(
        ast[0],
        Element::If(If {
            conditions: vec![
                Condition {
                    kwd_span: None,
                    condition: Some(*ident!(1, 4, "x")),
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                },
                Condition {
                    kwd_span: None,
                    condition: Some(*ident!(1, 15, "y")),
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                }
            ]
        })
    )
}

#[test]
fn if_elif_else() {
    let ast = parse!("if x { } elif y { } else { }");
    assert_eq!(
        ast[0],
        Element::If(If {
            conditions: vec![
                Condition {
                    kwd_span: None,
                    condition: Some(*ident!(1, 4, "x")),
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                },
                Condition {
                    kwd_span: None,
                    condition: Some(*ident!(1, 15, "y")),
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                },
                Condition {
                    kwd_span: None,
                    condition: None,
                    if_true: Block {
                        brace_spans: None,
                        content: vec![]
                    }
                }
            ]
        })
    )
}

#[test]
fn parentheses() {
    let ast = parse!("(x)");
    assert_eq!(ast[0], *ident!(1, 2, "x"))
}

#[test]
fn block() {
    let ast = parse!("{x}");
    assert_eq!(
        ast[0],
        Element::Block(Block {
            brace_spans: None,
            content: vec![*ident!(1, 2, "x")]
        })
    )
}

#[test]
fn preprocess_block() {
    let ast = parse!("pre {x}");
    assert_eq!(
        ast[0],
        Element::Preprocess(Preprocess {
            kwd_span: span!(1, 1, "pre"),
            content: Element::Block(Block {
                brace_spans: None,
                content: vec![*ident!(1, 6, "x")]
            })
            .into()
        })
    )
}

#[test]
fn preprocess_expr() {
    let ast = parse!("pre x");
    assert_eq!(
        ast[0],
        Element::Preprocess(Preprocess {
            kwd_span: span!(1, 1, "pre"),
            content: ident!(1, 5, "x")
        })
    )
}

#[test]
fn defer_block() {
    let ast = parse!("defer {x}");
    assert_eq!(
        ast[0],
        Element::Defer(Defer {
            kwd_span: span!(1, 1, "defer"),
            content: Element::Block(Block {
                brace_spans: None,
                content: vec![*ident!(1, 8, "x")]
            })
            .into()
        })
    )
}

#[test]
fn defer_expr() {
    let ast = parse!("defer x");
    assert_eq!(
        ast[0],
        Element::Defer(Defer {
            kwd_span: span!(1, 1, "defer"),
            content: ident!(1, 7, "x")
        })
    )
}

#[test]
fn proc_kwd() {
    let ast = parse!("proc | | x");
    assert_eq!(
        ast[0],
        Element::Procedure(Procedure {
            is_fn: false,
            kwd_span: Some(span!(1, 1, "proc")),
            args: vec![],
            return_type: None,
            content: Block {
                brace_spans: None,
                content: vec![*ident!(1, 10, "x")]
            }
        })
    )
}

#[test]
fn proc_nokwd() {
    let ast = parse!("| | x");
    assert_eq!(
        ast[0],
        Element::Procedure(Procedure {
            is_fn: false,
            kwd_span: None,
            args: vec![],
            return_type: None,
            content: Block {
                brace_spans: None,
                content: vec![*ident!(1, 5, "x")]
            }
        })
    )
}

#[test]
fn fn_kwd() {
    let ast = parse!("fn | | x");
    assert_eq!(
        ast[0],
        Element::Procedure(Procedure {
            is_fn: true,
            kwd_span: Some(span!(1, 1, "fn")),
            args: vec![],
            return_type: None,
            content: Block {
                brace_spans: None,
                content: vec![*ident!(1, 8, "x")]
            }
        })
    )
}

#[test]
fn fn_arg() {
    let ast = parse!("fn | | x");
    assert_eq!(
        ast[0],
        Element::Procedure(Procedure {
            is_fn: true,
            kwd_span: Some(span!(1, 1, "fn")),
            args: vec![],
            return_type: None,
            content: Block {
                brace_spans: None,
                content: vec![*ident!(1, 8, "x")]
            }
        })
    )
}

#[test]
fn return_nothing() {
    let ast = parse!("ret");
    assert_eq!(
        ast[0],
        Element::Return(Return {
            kwd_span: Some(span!(1, 1, "ret")),
            value: UNIT_T.as_type().as_type_element().as_literal().into()
        })
    )
}

#[test]
fn return_something() {
    let ast = parse!("ret x");
    assert_eq!(
        ast[0],
        Element::Return(Return {
            kwd_span: Some(span!(1, 1, "ret")),
            value: ident!(1, 5, "x")
        })
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
            Element::UnaryOpr(UnaryOpr {
                ty,
                opr_span: Some(span!(1, 1, "-")),
                operand: ident!(1, 2, "x"),
            })
        )
    }
}

#[test]
fn unparen_call_single() {
    let ast = parse!("x y");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: ident!(1, 1, "x"),
            paren_spans: None,
            args: vec![*ident!(1, 3, "y")],
            kwargs: Default::default()
        })
    )
}

#[test]
fn unparen_call_multiple() {
    let ast = parse!("x y, z");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: ident!(1, 1, "x"),
            paren_spans: None,
            args: vec![*ident!(1, 3, "y"), *ident!(1, 6, "z")],
            kwargs: Default::default()
        })
    )
}

#[test]
fn unparen_call_nested() {
    let ast = parse!("x y z");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: ident!(1, 1, "x"),
            paren_spans: None,
            args: vec![Element::Call(Call {
                called: ident!(1, 3, "y"),
                paren_spans: None,
                args: vec![*ident!(1, 5, "z")],
                kwargs: Default::default()
            })],
            kwargs: Default::default()
        })
    )
}

#[test]
fn dot() {
    let ast = parse!("x.y");
    assert_eq!(
        ast[0],
        Element::Ident(Ident {
            name: "y".into(),
            name_span: Some(span!(1, 3, "y")),
            dot_span: Some(span!(1, 2, ".")),
            parent: Some(ident!(1, 1, "x"))
        })
    )
}

#[test]
fn call_no_args() {
    let ast = parse!("x()");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: ident!(1, 1, "x"),
            paren_spans: Some((span!(1, 2, "("), span!(1, 3, ")"))),
            args: vec![],
            kwargs: Default::default()
        })
    )
}

#[test]
fn call_with_args() {
    let ast = parse!("x(y)");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: ident!(1, 1, "x"),
            paren_spans: Some((span!(1, 2, "("), span!(1, 4, ")"))),
            args: vec![*ident!(1, 3, "y")],
            kwargs: Default::default()
        })
    )
}

#[test]
fn dot_call() {
    let ast = parse!("x.y()");
    assert_eq!(
        ast[0],
        Element::Call(Call {
            called: Box::new(Element::Ident(Ident {
                name: "y".into(),
                name_span: Some(span!(1, 3, "y")),
                dot_span: Some(span!(1, 2, ".")),
                parent: Some(ident!(1, 1, "x")),
            })),
            paren_spans: Some((span!(1, 4, "("), span!(1, 5, ")"))),
            args: vec![],
            kwargs: Default::default()
        })
    )
}
