use std::{cmp::min, collections::HashMap};

use num::BigInt;

use crate::{
    types::{
        element::{
            block::Block, ident::Ident, procedure::Argument, r#if::Condition, Element, ElementData,
            ElementVariant, VecElementRaw,
        },
        errors::ZyxtError,
        token::{get_order, Keyword, OprType, Side, Token, TokenCategory, TokenType},
        value::Value,
    },
    Type,
};
/*
macro_rules! check_and_update_cursor {
    ($cursor: ident, $selected: ident, $elements: ident) => {
        if $cursor == $elements.len() - 1 {
            return Err(ZyxtError::error_2_1_0($selected.pos_raw.raw)
                .with_pos_raw($selected.get_pos(), &$selected.pos_raw.raw));
        }
        $cursor += 1;
        $selected = &$elements[$cursor];
    };
}

fn catch_between(
    opening: Option<TokenType>,
    closing: Option<TokenType>,
    elements: &[Element],
    cursor: &mut usize,
) -> Result<Vec<Element>, ZyxtError> {
    let mut paren_level = 0;
    let mut catcher: Vec<Element> = vec![];
    let opening_char = match opening {
        Some(TokenType::OpenParen) => '(',
        Some(TokenType::OpenSquareParen) => '[',
        Some(TokenType::OpenCurlyParen) => '{',
        Some(TokenType::OpenAngleBracket) => '<',
        _ => '?',
    };
    let paren_pos = elements[*cursor].get_pos().to_owned();
    loop {
        *cursor += 1;
        if *cursor >= elements.len() {
            return if opening.is_none() {
                Err(ZyxtError::error_2_1_0(elements[*cursor].pos_raw.raw)
                    .with_pos_raw(&paren_pos, &elements[*cursor].pos_raw.raw))
            } else {
                Err(ZyxtError::error_2_0_1(opening_char.to_string())
                    .with_pos_raw(&paren_pos, &opening_char.to_string()))
            };
        }
        let catcher_selected = &elements[*cursor];
        if let Element::Token(catcher_selected) = catcher_selected {
            if catcher_selected.ty == closing && paren_level == 0 {
                break;
            } else if catcher_selected.ty == closing {
                paren_level -= 1;
            } else if catcher_selected.ty == opening {
                paren_level += 1;
            }
        }
        catcher.push(catcher_selected.to_owned())
    }
    Ok(catcher)
}

fn base_split<T: Clone>(
    parser_fn: &dyn Fn(Vec<Element>) -> Result<T, ZyxtError>,
    default_val: Option<T>,
    divider: TokenType,
    opening: Option<TokenType>,
    closing: Option<TokenType>,
    elements: Vec<Element>,
    ignore_empty: bool,
) -> Result<Vec<T>, ZyxtError> {
    let mut out: Vec<T> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut paren_level = 0;
    for element in elements {
        if let Element::Token(Token { ty, .. }) = element {
            if ty == Some(divider) && paren_level == 0 {
                if !ignore_empty && catcher.is_empty() {
                    todo!()
                } else if catcher.is_empty() {
                    out.push(default_val.to_owned().unwrap())
                } else if !catcher.is_empty() {
                    out.push(parser_fn(catcher.to_owned())?);
                }
                catcher.clear();
            } else {
                if ty == opening {
                    paren_level += 1;
                } else if ty == closing {
                    paren_level -= 1;
                }
                catcher.push(element.to_owned());
            }
        } else {
            catcher.push(element.to_owned());
        }
    }
    if paren_level != 0 {
        return Err(ZyxtError::error_2_0_1(
            match opening {
                Some(TokenType::OpenParen) => '(',
                Some(TokenType::OpenSquareParen) => '[',
                Some(TokenType::OpenCurlyParen) => '{',
                Some(TokenType::OpenAngleBracket) => '<',
                _ => '?',
            }
            .to_string(),
        )
        .with_pos_raw(&Default::default(), &"".to_string())); // TODO
    }
    if out.is_empty() && catcher.is_empty() {
        return Ok(vec![]);
    }
    if !ignore_empty && catcher.is_empty() {
        todo!()
    } else if catcher.is_empty() {
        out.push(default_val.unwrap())
    } else if !catcher.is_empty() {
        out.push(parser_fn(catcher.to_owned())?);
    }
    Ok(out)
}

fn split_between(
    divider: TokenType,
    opening: Option<TokenType>,
    closing: Option<TokenType>,
    elements: Vec<Element>,
    ignore_empty: bool,
) -> Result<Vec<Element>, ZyxtError> {
    base_split(
        &parse_expr,
        Some(Element::NullElement),
        divider,
        opening,
        closing,
        elements,
        ignore_empty,
    )
}

fn get_arguments(
    cursor: &mut usize,
    elements: &[Element],
    raw: &mut String,
) -> Result<Vec<Argument>, ZyxtError> {
    let contents = catch_between(Some(TokenType::Bar), Some(TokenType::Bar), elements, cursor)?;
    *raw = format!(
        "{}{}{}",
        raw,
        contents
            .iter()
            .map(|e| e.pos_raw.raw)
            .collect::<Vec<String>>()
            .join(""),
        elements[*cursor].pos_raw.raw
    );
    base_split(
        &|raw_arg| {
            let parts = split_between(TokenType::Colon, None, None, raw_arg.to_owned(), true)?;
            let name = if let Some(Element { data: ElementVariant::Ident(ident), ..}) = parts.get(0) {
                ident.name.to_owned()
            } else {
                return Err(ZyxtError::error_2_1_15(",".to_string())
                    .with_pos_raw(parts.get(0).unwrap().get_pos(), &raw_arg.pos_raw.raw));
            };
            let ty = parts.get(1).cloned();
            let default = parts.get(2).cloned();
            if parts.len() > 3 {
                return Err(ZyxtError::error_2_1_14(parts[3].pos_raw.raw)
                    .with_pos_raw(parts.get(3).unwrap().get_pos(), &raw_arg.pos_raw.raw));
            }
            Ok(Argument {
                name,
                ty: ty.unwrap_or(Element {
                    pos_raw: Default::default(),
                    data:ty.unwrap_or(Element {
                        pos_raw: Default::default(),
                        data: Box::new(Ident {
                            name: "_any".into(),
                            parent: None
                        }.as_variant())
                    }).data
                }),
                default,
            })
        },
        None,
        TokenType::Comma,
        Some(TokenType::Bar),
        Some(TokenType::Bar),
        contents,
        false,
    )
}

fn parse_parens(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            if selected.ty == Some(TokenType::OpenParen) {
                let mut prev_element = &Element::Token(Token {
                    ..Default::default()
                });
                if cursor != 0 {
                    prev_element = &elements[cursor - 1];
                }
                if let Element::Token(prev_element) = prev_element {
                    // if selected is Token and is (
                    if cursor == 0
                        || if let Some(ty) = prev_element.ty {
                            !ty.categories().contains(&TokenCategory::ValueEnd)
                        } else {
                            true
                        }
                    {
                        let paren_contents = catch_between(
                            Some(TokenType::OpenParen),
                            Some(TokenType::CloseParen),
                            &elements,
                            &mut cursor,
                        )?;
                        new_elements.push(parse_expr(paren_contents)?);
                        if let Some(raw) = new_elements.last_mut().unwrap().get_raw_mut() {
                            *raw = format!(
                                "{}{}{}",
                                selected.pos_raw.raw,
                                raw,
                                elements[cursor].pos_raw.raw
                            );
                        }
                    } else {
                        new_elements.push(Element::Token(selected.to_owned()))
                    } // or else it's function args
                } else {
                    new_elements.push(Element::Token(selected.to_owned()))
                }
            } else if selected.ty == Some(TokenType::OpenCurlyParen) {
                // blocks, {
                let raw = selected.pos_raw.raw;
                let paren_contents = catch_between(
                    Some(TokenType::OpenCurlyParen),
                    Some(TokenType::CloseCurlyParen),
                    &elements,
                    &mut cursor,
                )?;
                new_elements.push(Element::Block {
                    position: selected.position.to_owned(),
                    raw: format!(
                        "{}{}{}",
                        raw,
                        paren_contents
                            .iter()
                            .map(|e| e.pos_raw.raw)
                            .collect::<Vec<String>>()
                            .join(""),
                        elements[cursor].pos_raw.raw
                    ),
                    content: parse_block(paren_contents)?,
                });
            } else {
                new_elements.push(Element::Token(selected.to_owned()))
            }
        } else {
            new_elements.push(selected.to_owned())
        }
        cursor += 1;
    }

    Ok(new_elements)
}

fn parse_preprocess_and_defer(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements = vec![];

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token {
            ty: Some(TokenType::Keyword(Keyword::Pre)),
            position,
            value,
            ..
        })
        | Element::Token(Token {
            ty: Some(TokenType::Keyword(Keyword::Defer)),
            position,
            value,
            ..
        }) = selected
        {
            if cursor == elements.len() - 1 {
                return Err(
                    ZyxtError::error_2_1_16().with_pos_raw(position, &value.to_string())
                );
            }
            let raw = selected.pos_raw.raw;
            let is_pre = raw.trim() == "pre";
            check_and_update_cursor!(cursor, selected, elements);
            if let Element::Block {
                content,
                raw: block_raw,
                ..
            } = selected
            {
                if is_pre {
                    new_elements.push(Element::Preprocess {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, block_raw),
                        content: content.to_owned(),
                    })
                } else {
                    new_elements.push(Element::Defer {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, block_raw),
                        content: content.to_owned(),
                    })
                }
            } else {
                let content = parse_expr(elements[cursor..].to_vec())?;
                if is_pre {
                    new_elements.push(Element::Preprocess {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, content.pos_raw.raw),
                        content: vec![content],
                    });
                } else {
                    new_elements.push(Element::Defer {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, content.pos_raw.raw),
                        content: vec![content],
                    })
                }
                break;
            }
        } else {
            new_elements.push(selected.to_owned())
        }
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_classes_structs_and_mixins(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements = vec![];

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token {
            ty: Some(TokenType::Keyword(keyword)),
            position,
            ..
        }) = selected
        {
            if [Keyword::Class, Keyword::Struct].contains(keyword) {
                let mut raw = selected.pos_raw.raw;
                check_and_update_cursor!(cursor, selected, elements);

                let mut args = None;
                if let Element::Token(Token {
                    ty: Some(TokenType::Bar),
                    position,
                    value,
                    ..
                }) = selected
                {
                    if keyword == &Keyword::Class {
                        return Err(ZyxtError::error_2_1_17()
                            .with_pos_raw(position, &format!("class {}", value.trim())));
                    }
                    args = Some(get_arguments(&mut cursor, &elements, &mut raw)?);
                    check_and_update_cursor!(cursor, selected, elements);
                }
                let mut content = vec![];
                if let ElementVariant::Block(Block {
                    content: block_content,
                    ..
                }) = &selected.data
                {
                    content = block_content.to_owned();
                    raw = format!("{}{}", raw, selected.pos_raw.raw);
                } else if keyword == &Keyword::Class {
                    return Err(ZyxtError::error_2_1_18(keyword).with_pos_raw(
                        selected.get_pos(),
                        &format!("{}{}", raw, &selected.pos_raw.raw),
                    ));
                }
                new_elements.push(Element::Class {
                    position: position.to_owned(),
                    raw,
                    is_struct: keyword == &Keyword::Struct,
                    implementations: Default::default(),
                    inst_fields: Default::default(),
                    content,
                    args,
                })
            } else {
                new_elements.push(selected.to_owned())
            }
        } else {
            new_elements.push(selected.to_owned())
        }
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_vars_literals_and_calls(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut catcher: Element = Element::NullElement;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            match selected.ty {
                Some(TokenType::DotOpr) => {
                    // TODO rewrite this
                    if cursor == 0 {
                        return Err(ZyxtError::error_2_1_0(String::from("."))
                            .with_element(&elements[cursor])); // could be enum but that's for later
                    } else if cursor == elements.len() - 1 {
                        return Err(ZyxtError::error_2_1_2().with_element(&elements[cursor]));
                        // definitely at the wrong place
                    }
                    let prev_element = &elements[cursor - 1];
                    let next_element = &elements[cursor + 1];
                    if let Element::Token(next_element) = next_element {
                        if next_element.ty != Some(TokenType::Ident) {
                            return Err(ZyxtError::error_2_1_0(next_element.value.to_owned())
                                .with_token(next_element));
                        }
                    } else if let Element::Literal { content, .. } = next_element {
                        return Err(
                            ZyxtError::error_2_1_0(content.to_owned()).with_element(next_element)
                        );
                    }
                    if let (Element::Token(prev_element), Element::Token(next_element)) =
                        (prev_element, next_element)
                    {
                        if if let Some(ty) = prev_element.ty {
                            !ty.categories().contains(&TokenCategory::ValueEnd)
                        } else {
                            true
                        } {
                            return Err(
                                ZyxtError::error_2_1_0(String::from(".")).with_token(selected)
                            ); //could be enum but that's for later
                        }
                        catcher = Element::Ident {
                            position: next_element.position.to_owned(),
                            name: next_element.value.to_owned(),
                            raw: format!(
                                "{}{}{}",
                                catcher.pos_raw.raw,
                                selected.pos_raw.raw,
                                next_element.pos_raw.raw
                            ),
                            parent: Box::new(catcher),
                        };
                        cursor += 1;
                    } else if let Element::Token(next_element) = next_element {
                        catcher = Element::Ident {
                            position: next_element.position.to_owned(),
                            name: next_element.value.to_owned(),
                            raw: format!(
                                "{}{}{}",
                                catcher.pos_raw.raw,
                                selected.pos_raw.raw,
                                next_element.pos_raw.raw
                            ),
                            parent: Box::new(catcher),
                        };
                    } else {
                        return Err(ZyxtError::error_2_1_0(String::from(".")).with_token(selected));
                        // definitely at the wrong place
                    }
                }
                Some(TokenType::Ident) => {
                    if catcher != Element::NullElement {
                        new_elements.push(catcher.to_owned());
                    }
                    catcher = Element::Ident {
                        position: selected.position.to_owned(),
                        name: selected.value.to_owned(),
                        raw: selected.pos_raw.raw,
                        parent: Box::new(Element::NullElement),
                    }
                }
                Some(TokenType::LiteralNumber)
                | Some(TokenType::LiteralMisc)
                | Some(TokenType::LiteralString) => {
                    if catcher != Element::NullElement {
                        new_elements.push(catcher.to_owned());
                    }
                    catcher = Element::Literal {
                        position: selected.position.to_owned(),
                        raw: selected.pos_raw.raw,
                        content: match selected.ty {
                            Some(TokenType::LiteralMisc) => match &*selected.value {
                                "true" => Value::Bool(true),
                                "false" => Value::Bool(false),
                                "unit" => todo!(),
                                "inf" => Value::F64(f64::INFINITY),
                                _ => unreachable!("{}", selected.value),
                            },
                            Some(TokenType::LiteralNumber) => {
                                if selected.value.contains('.') {
                                    Value::F64(selected.value.parse().unwrap()) // TODO Decimal
                                } else if let Ok(val) = selected.value.parse::<i32>() {
                                    Value::I32(val)
                                } else if let Ok(val) = selected.value.parse::<i64>() {
                                    Value::I64(val)
                                } else if let Ok(val) = selected.value.parse::<i128>() {
                                    Value::I128(val)
                                } else if let Ok(val) = selected.value.parse::<u128>() {
                                    Value::U128(val)
                                } else if let Ok(val) = selected.value.parse::<BigInt>() {
                                    Value::Ibig(val)
                                } else {
                                    unreachable!()
                                }
                            }
                            Some(TokenType::LiteralString) => {
                                Value::Str(selected.value[1..selected.value.len() - 1].to_string())
                            }
                            ty => unreachable!("{ty:?}"),
                        }, /*ty: Type::from_name(if selected.ty == TokenType::LiteralMisc {
                               match &*selected.value {
                                   "true" | "false" => "bool",
                                   "null" => "_unit",
                                   "inf" |  p if p == *Undef_T => "_num",
                                   _ => unreachable!("{}", selected.value),
                               }
                           } else if selected.ty == TokenType::LiteralNumber {
                               if selected.value.contains('.') {
                                   "f64"
                               } else if selected.value.parse::<i32>().is_ok() {
                                   "i32"
                               } else if selected.value.parse::<i64>().is_ok() {
                                   "i64"
                               } else if selected.value.parse::<i128>().is_ok() {
                                   "i128"
                               } else if selected.value.parse::<u128>().is_ok() {
                                   "u128"
                               } else {
                                   "ibig"
                               }
                           } else {
                               "str"
                           }),
                           content: if selected.ty == TokenType::LiteralString {
                               selected.value[1..selected.value.len() - 1].to_string()
                           } else {
                               selected.value.to_owned()
                           },*/
                    }
                }
                Some(TokenType::CloseParen) => {
                    return Err(ZyxtError::error_2_0_2(')'.to_string()).with_token(selected))
                }
                Some(TokenType::OpenParen) => {
                    if cursor == 0 {
                        return Err(ZyxtError::error_2_1_0(String::from("(")).with_token(selected));
                        // parens should have been settled in the first part
                    }
                    let mut raw = selected.pos_raw.raw;
                    let contents = catch_between(
                        Some(TokenType::OpenParen),
                        Some(TokenType::CloseParen),
                        &elements,
                        &mut cursor,
                    )?;
                    raw = format!(
                        "{}{}{}",
                        raw,
                        contents
                            .iter()
                            .map(|e| e.pos_raw.raw)
                            .collect::<Vec<String>>()
                            .join(""),
                        elements[cursor].pos_raw.raw
                    );
                    let args = split_between(
                        TokenType::Comma,
                        Some(TokenType::OpenParen),
                        Some(TokenType::CloseParen),
                        contents,
                        false,
                    )?;
                    catcher = Element::Call {
                        position: catcher.get_pos().to_owned(),
                        raw: format!("{}{}", catcher.pos_raw.raw, raw),
                        called: Box::new(catcher),
                        args,
                        kwargs: HashMap::new(),
                    }
                }
                _ => {
                    if catcher != Element::NullElement {
                        new_elements.push(catcher.to_owned());
                    }
                    catcher = Element::NullElement;
                    new_elements.push(Element::Token(selected.to_owned()));
                }
            }
        } else {
            if catcher != Element::NullElement {
                new_elements.push(catcher.to_owned());
            }
            catcher = selected.to_owned()
        }
        cursor += 1;
    }
    if catcher != Element::NullElement {
        new_elements.push(catcher);
    }
    //catcher2 = Element::NullElement;
    Ok(new_elements)
}

fn parse_procs_and_fns(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];
    let mut cursor = 0;
    let mut selected;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token { ty, .. }) = selected {
            if [
                Some(TokenType::Keyword(Keyword::Proc)),
                Some(TokenType::Keyword(Keyword::Fn)),
                Some(TokenType::Bar),
            ]
            .contains(ty)
            {
                let position = selected.get_pos().to_owned();
                let is_fn = if *ty != Some(TokenType::Bar) {
                    *ty == Some(TokenType::Keyword(Keyword::Fn))
                } else {
                    false
                };
                let mut raw = selected.pos_raw.raw.to_owned();
                if *ty != Some(TokenType::Bar) {
                    check_and_update_cursor!(cursor, selected, elements);
                    raw = format!("{}{}", raw, selected.pos_raw.raw);
                }

                let args = if let Element::Token(Token {
                    ty: Some(TokenType::Bar),
                    ..
                }) = selected
                {
                    get_arguments(&mut cursor, &elements, &mut raw)?
                } else {
                    cursor -= 1;
                    vec![]
                };

                check_and_update_cursor!(cursor, selected, elements);
                let return_type = Box::new(
                    if let Element::Token(Token {
                        ty: Some(TokenType::Colon),
                        value,
                        ..
                    }) = selected
                    {
                        let mut catcher = vec![];
                        raw = format!("{}{}", raw, value);
                        loop {
                            check_and_update_cursor!(cursor, selected, elements);
                            raw = format!("{}{}", raw, selected.pos_raw.raw);
                            if let Element::Block { .. } = selected {
                                break;
                            }
                            catcher.push(selected.to_owned());
                        }
                        let return_type = parse_expr(catcher)?;
                        if matches!(return_type.to_owned(), Element::Ident { .. }) {
                            return_type
                        } else {
                            todo!("throw error here")
                        }
                    } else {
                        Element::Ident {
                            position: Default::default(),
                            raw: Default::default(),
                            name: "_unit".into(),
                            parent: Box::new(Element::NullElement),
                        }
                    },
                );

                if let Element::Block { content, .. } = selected {
                    new_elements.push(Element::Procedure {
                        position,
                        is_fn,
                        args,
                        return_type,
                        raw,
                        content: content.to_owned(),
                    });
                } else {
                    let content = parse_expr(elements[cursor..].to_vec())?;
                    new_elements.push(Element::Procedure {
                        position,
                        is_fn,
                        args,
                        return_type,
                        raw: format!("{}{}", raw, content.pos_raw.raw),
                        content: vec![content],
                    });
                    return Ok(new_elements);
                }
            } else {
                new_elements.push(selected.to_owned())
            }
        } else {
            new_elements.push(selected.to_owned())
        }
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_assignment_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {
        return Ok(vec![]);
    }
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token {
            ty: Some(TokenType::AssignmentOpr(opr_type)),
            position,
            ..
        }) = ele
        {
            if i == 0 || i == elements.len() - 1 {
                return Err(ZyxtError::error_2_1_3(ele.pos_raw.raw).with_element(ele));
            }
            let variable = parse_expr(vec![elements[i - 1].to_owned()])?;
            let content = if let Some(opr_type) = opr_type {
                let operand2 = parse_expr(elements[i + 1..].to_vec())?;
                Element::BinaryOpr {
                    position: position.to_owned(),
                    ty: *opr_type,
                    raw: operand2.pos_raw.raw,
                    operand1: Box::new(variable.to_owned()),
                    operand2: Box::new(operand2),
                }
            } else {
                parse_expr(elements[i + 1..].to_vec())?
            };

            return Ok(elements[..i - 1]
                .iter()
                .cloned()
                .chain(vec![Element::Set {
                    position: position.to_owned(),
                    raw: format!(
                        "{}{}{}",
                        variable.pos_raw.raw,
                        ele.pos_raw.raw,
                        content.pos_raw.raw
                    ),
                    variable: Box::new(variable),
                    content: Box::new(content),
                }])
                .collect::<Vec<Element>>());
        }
    }
    Ok(elements)
}

fn parse_un_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {
        return Ok(vec![]);
    }
    for (i, ele) in elements.iter().enumerate().rev() {
        if let Element::Token(Token {
            ty: Some(TokenType::UnaryOpr(opr_type)),
            position,
            ..
        }) = ele
        {
            if opr_type.side() == Side::Left {
                if i == elements.len() - 1 {
                    return Err(ZyxtError::error_2_1_4(ele.pos_raw.raw).with_element(ele));
                }
                let operand = parse_un_oprs(elements[i + 1..].to_vec())?[0].to_owned();
                return parse_un_oprs(
                    elements[..i]
                        .iter()
                        .cloned()
                        .chain(vec![Element::UnaryOpr {
                            position: position.to_owned(),
                            ty: *opr_type,
                            raw: format!("{}{}", ele.pos_raw.raw, operand.pos_raw.raw),
                            operand: Box::new(operand),
                        }])
                        .collect::<Vec<Element>>(),
                );
            } else if opr_type.side() == Side::Right {
                if i == 0 {
                    return Err(ZyxtError::error_2_1_4(ele.pos_raw.raw).with_element(ele));
                }
                let operand = parse_un_oprs(elements[..i].to_vec())?[0].to_owned();
                return parse_un_oprs(
                    vec![Element::UnaryOpr {
                        position: position.to_owned(),
                        ty: *opr_type,
                        raw: format!("{}{}", operand.pos_raw.raw, ele.pos_raw.raw),
                        operand: Box::new(operand),
                    }]
                    .into_iter()
                    .chain(elements[i + 1..].to_vec())
                    .collect::<Vec<Element>>(),
                );
            }
        }
    }
    Ok(elements)
}

fn parse_normal_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {
        return Ok(vec![]);
    }
    let mut highest_order_index: usize = 0;
    let mut highest_order = 0;
    let mut opr_detected = false;
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token {
            ty: Some(TokenType::NormalOpr(opr_type)),
            value,
            ..
        }) = ele
        {
            if i == 0 || i == elements.len() - 1 {
                return Err(ZyxtError::error_2_1_3(value.to_owned()).with_element(ele));
            }
            if get_order(opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(opr_type);
                opr_detected = true
            }
        }
    }
    Ok(if !opr_detected {
        elements
    } else if let Element::Token(Token {
        ty: Some(TokenType::NormalOpr(opr_type)),
        position,
        ..
    }) = &elements[highest_order_index]
    {
        let operand1 = parse_expr(elements[..highest_order_index].to_vec())?;
        let operand2 = parse_expr(elements[highest_order_index + 1..].to_vec())?;
        vec![Element::BinaryOpr {
            position: position.to_owned(),
            ty: *opr_type,
            raw: format!(
                "{}{}{}",
                operand1.pos_raw.raw,
                elements[highest_order_index].pos_raw.raw,
                operand2.pos_raw.raw
            ),
            operand1: Box::new(operand1),
            operand2: Box::new(operand2),
        }]
    } else {
        elements
    })
}

fn parse_delete_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token {
            ty: Some(TokenType::Keyword(Keyword::Delete)),
            ..
        }) = ele
        {
            let vars_to_delete = split_between(
                TokenType::Comma,
                None,
                None,
                elements[i + 1..].to_vec(),
                false,
            )?;
            let mut varnames = vec![];
            for var in vars_to_delete.iter() {
                if let Element::Ident { name, .. } = var {
                    varnames.push(name.to_owned());
                } else if let Element::UnaryOpr {
                    ty: OprType::Deref,
                    raw,
                    ..
                } = var
                {
                    return Err(ZyxtError::error_2_1_12(raw.to_owned()).with_element(var));
                } else {
                    return Err(ZyxtError::error_2_1_11(var.pos_raw.raw).with_element(var));
                }
            }
            new_elements.push(Element::Delete {
                position: ele.get_pos().to_owned(),
                raw: format!(
                    "{}{}",
                    ele.pos_raw.raw,
                    elements[i + 1..]
                        .iter()
                        .map(|e| e.pos_raw.raw)
                        .collect::<Vec<String>>()
                        .join("")
                ),
                names: varnames,
            });
            return Ok(new_elements);
        }
        new_elements.push(ele.to_owned());
    }
    Ok(elements)
}

fn parse_return_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token {
            ty: Some(TokenType::Keyword(Keyword::Return)),
            whitespace,
            value,
            ..
        }) = ele
        {
            let return_val = parse_expr(elements[i + 1..].to_vec())?;
            new_elements.push(Element::Return {
                position: ele.get_pos().to_owned(),
                raw: format!("{}{}{}", whitespace, value, return_val.pos_raw.raw),
                value: Box::new(return_val),
            });
            return Ok(new_elements);
        }
        new_elements.push(ele.to_owned());
    }
    Ok(elements)
}

fn parse_declaration_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    let mut flag_pos = None;
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token {
            ty: Some(TokenType::Flag(_)),
            ..
        }) = selected
        {
            flag_pos = Some(cursor);
        }
        if let Element::Token(Token {
            ty: Some(TokenType::DeclarationOpr),
            position,
            whitespace,
            value,
            ..
        }) = selected
        {
            if cursor == elements.len() - 1 || cursor == 0 {
                return Err(ZyxtError::error_2_1_5().with_element(selected));
            }
            let declared_var: &Element = &elements[cursor - 1];
            let mut raw = format!("{}{}{}", declared_var.pos_raw.raw, whitespace, value);
            let flags = if flag_pos == None {
                vec![]
            } else {
                let mut f = vec![];
                for ele in elements[flag_pos.unwrap()..cursor - 1].iter() {
                    if let Element::Token(Token {
                        ty: Some(TokenType::Flag(flag)),
                        whitespace,
                        value,
                        ..
                    }) = &ele
                    {
                        raw = format!("{}{}{}", whitespace, value, raw);
                        f.push(*flag);
                    } else {
                        return Err(ZyxtError::error_2_1_6(ele.pos_raw.raw).with_element(ele));
                    }
                }
                f
            };
            for _ in 0..flags.len() + 1 {
                new_elements.pop();
            }
            let content = parse_expr(elements[cursor + 1..].to_vec())?;
            new_elements.push(Element::Declare {
                position: position.to_owned(),
                raw: format!("{}{}", raw, content.pos_raw.raw),
                variable: Box::new(parse_expr(vec![declared_var.to_owned()])?),
                content: Box::new(content),
                flags,
                ty: Box::new(Element::Literal {
                    position: Default::default(),
                    raw: Default::default(),
                    content: Value::Type(Type::Any),
                }), // TODO type later
            });
            break;
        } else {
            new_elements.push(selected.to_owned())
        }
        cursor += 1;
    }
    Ok(new_elements)
}

pub fn parse_if_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token {
            ty: Some(TokenType::Keyword(kwd)),
            position,
            ..
        }) = selected
        {
            match kwd {
                Keyword::If => {
                    let start_pos = position.to_owned();
                    let mut conditions: Vec<Condition> = vec![];
                    let mut prev_catcher_kwd = "";
                    let mut raw = String::new();
                    loop {
                        let catcher_kwd;
                        let mut catcher_selected = &elements[cursor];
                        if let Element::Token(Token {
                            ty: Some(TokenType::Keyword(prekwd)),
                            position,
                            whitespace,
                            value,
                            ..
                        }) = catcher_selected
                        {
                            catcher_kwd = match prekwd {
                                Keyword::If if position == &start_pos => "if",
                                Keyword::Elif if prev_catcher_kwd != "else" => "elif",
                                Keyword::Else if prev_catcher_kwd != "else" => "else",
                                Keyword::Elif if prev_catcher_kwd == "else" => {
                                    return Err(ZyxtError::error_2_1_7("elif".to_string())
                                        .with_element(catcher_selected))
                                }
                                Keyword::Else if prev_catcher_kwd == "else" => {
                                    return Err(ZyxtError::error_2_1_7("else".to_string())
                                        .with_element(catcher_selected))
                                }
                                _ => break,
                            };
                            raw = format!("{}{}{}", raw, whitespace, value);
                        } else {
                            break;
                        }
                        prev_catcher_kwd = catcher_kwd;
                        check_and_update_cursor!(cursor, catcher_selected, elements);
                        let condition = if catcher_kwd == "else" {
                            None
                        } else if let Element::Block(block) = catcher_selected.data {
                            raw = format!("{}{}", raw, block.raw);
                            check_and_update_cursor!(cursor, catcher_selected, elements);
                            Some(catcher_selected.to_owned())
                        } else {
                            let mut catcher = vec![elements[cursor].to_owned()];
                            loop {
                                check_and_update_cursor!(cursor, catcher_selected, elements);
                                raw = format!("{}{}", raw, catcher_selected.pos_raw.raw);
                                if let Element::Block { .. } = catcher_selected {
                                    break;
                                } else {
                                    catcher.push(catcher_selected.to_owned());
                                }
                            }
                            Some(parse_expr(catcher)?)
                        };
                        catcher_selected = &elements[cursor];
                        raw = format!("{}{}", raw, catcher_selected.pos_raw.raw);
                        if let ElementVariant::Block(block) = catcher_selected {
                            conditions.push(Condition {
                                condition,
                                if_true: Element {
                                    pos_raw: catcher_selected.pos_raw.to_owned(),
                                    data: Box::new(block.to_owned())
                                },
                            })
                        } else {
                            return Err(ZyxtError::error_2_1_8(catcher_selected.pos_raw.raw)
                                .with_element(selected));
                        }
                        cursor += 1;
                        if cursor == elements.len() {
                            break;
                        }
                    }
                    new_elements.push(Element::If {
                        position: start_pos,
                        raw,
                        conditions,
                    });
                    cursor -= 1;
                }
                Keyword::Elif | Keyword::Else => {
                    return Err(ZyxtError::error_2_1_9(
                        if kwd == &Keyword::Elif {
                            "elif"
                        } else {
                            "else"
                        }
                        .to_string(),
                    )
                    .with_element(selected))
                }
                _ => new_elements.push(selected.to_owned()),
            }
        } else {
            new_elements.push(selected.to_owned());
        }
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_unparen_calls(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let comma_pos = elements
        .iter()
        .position(|e| {
            matches!(
                e,
                Element::Token(Token {
                    ty: Some(TokenType::Comma),
                    ..
                })
            )
        })
        .unwrap_or(elements.len());
    let right_un_pos = elements
        .iter()
        .enumerate()
        .take_while(|(i, _)| *i < comma_pos)
        .collect::<Vec<_>>()
        .iter()
        .rposition(|(_, e)| {
            if let Element::Token(Token {
                ty: Some(TokenType::UnaryOpr(ty)),
                ..
            }) = e
            {
                ty.side() == Side::Right
            } else {
                false
            }
        });
    if let Some(right_un_pos) = right_un_pos {
        if right_un_pos + 1 != comma_pos {
            let min_index = min(right_un_pos + 1, elements.len());
            return parse_unparen_calls(
                parse_un_oprs(elements[..min_index].to_vec())?
                    .into_iter()
                    .chain(elements[min_index..].iter().cloned())
                    .collect(),
            );
        }
    }
    let left_un_pos = elements
        .iter()
        .enumerate()
        .take_while(|(i, _)| *i < comma_pos)
        .collect::<Vec<_>>()
        .iter()
        .rposition(|(_, e)| {
            if let Element::Token(Token {
                ty: Some(TokenType::UnaryOpr(ty)),
                ..
            }) = e
            {
                ty.side() == Side::Left
            } else {
                false
            }
        });
    if let Some(left_un_pos) = left_un_pos {
        if left_un_pos < comma_pos {
            let min_index = min(left_un_pos + 2, elements.len());
            return parse_unparen_calls(
                parse_un_oprs(elements[..min_index].to_vec())?
                    .into_iter()
                    .chain(elements[min_index..].iter().cloned())
                    .collect(),
            );
        }
    }

    if elements.len() == 1 {
        return Ok(elements);
    }
    Ok(vec![Element::Call {
        position: elements[0].get_pos().to_owned(),
        raw: elements
            .iter()
            .map(|e| e.pos_raw.raw)
            .collect::<Vec<String>>()
            .join(""),
        called: Box::new(elements[0].to_owned()),
        args: split_between(TokenType::Comma, None, None, elements[1..].to_vec(), false)?,
        kwargs: Default::default(),
    }])
}*/

fn parse_expr(mut elements: Vec<Element>) -> Result<Element, ZyxtError> {
    if elements.len() > 1 {
        elements = parse_parens(elements)?;
    }
    elements = parse_if_expr(elements)?;
    elements = parse_procs_and_fns(elements)?;
    elements = parse_preprocess_and_defer(elements)?;
    elements = parse_classes_structs_and_mixins(elements)?;
    //elements = parse_enums(elements)?;
    elements = parse_vars_literals_and_calls(elements)?;
    elements = parse_delete_expr(elements)?;
    elements = parse_return_expr(elements)?;
    elements = parse_declaration_expr(elements)?;
    elements = parse_assignment_oprs(elements)?;
    elements = parse_normal_oprs(elements)?;
    if elements.len() > 1 {
        elements = parse_unparen_calls(elements)?;
    }
    elements = parse_un_oprs(elements)?;
    if elements.len() > 1 {
        return Err(ZyxtError::error_2_1_0(elements[1].pos_raw.raw).with_element(&elements[1]));
    }
    Ok(elements.get(0).unwrap_or(&Element::NullElement).to_owned())
}

fn parse_block(input: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    split_between(
        TokenType::StatementEnd,
        Some(TokenType::OpenCurlyParen),
        Some(TokenType::CloseCurlyParen),
        input,
        true,
    )
}

pub fn parse_token_list(mut input: Vec<Token>) -> Result<Vec<Element>, ZyxtError> {
    let mut comments: Vec<Element> = vec![];

    // detect & remove comments
    for token in input.iter() {
        if token.ty == Some(TokenType::Comment) {
            comments.push(Element::Comment {
                position: token.position.to_owned(),
                raw: token.pos_raw.raw,
                content: token.value.to_owned(),
            })
        } else if [
            Some(TokenType::CommentStart),
            Some(TokenType::CommentEnd),
            Some(TokenType::MultilineCommentStart),
            Some(TokenType::MultilineCommentEnd),
        ]
        .contains(&token.ty)
        {
            return Err(ZyxtError::error_2_1_10(token.value.to_owned()).with_token(token));
        }
    }

    input.retain(|token| token.ty != Some(TokenType::Comment));

    // generate and return an AST for each expression
    parse_block(
        input
            .into_iter()
            .map(Element::Token)
            .collect::<Vec<Element>>(),
    )
}
