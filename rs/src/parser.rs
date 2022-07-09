use std::cmp::min;
use std::collections::HashMap;
use crate::objects::token::{TokenCategory, TokenType, get_order, Side, OprType, Keyword};
use crate::objects::element::{Argument, Condition, Element, VecElementRaw};
use crate::{Token, ZyxtError};
use crate::objects::typeobj::Type;

macro_rules! check_and_update_cursor {
    ($cursor: ident, $selected: ident, $elements: ident) => {
        if $cursor == $elements.len()-1 {
            return Err(ZyxtError::from_pos_and_raw($selected.get_pos(), &$selected.get_raw()).error_2_1_0($selected.get_raw()))
        }
        $cursor += 1;
        $selected = &$elements[$cursor];
    }
}

fn catch_between(opening: TokenType, closing: TokenType,
                 elements: &[Element], cursor: &mut usize) -> Result<Vec<Element>, ZyxtError> {
    let mut paren_level = 0;
    let mut catcher: Vec<Element> = vec![];
    let opening_char = match opening {
        TokenType::OpenParen => '(',
        TokenType::OpenSquareParen => '[',
        TokenType::OpenCurlyParen => '{',
        TokenType::OpenAngleBracket => '<',
        _ => '?'
    };
    let paren_pos = elements[*cursor].get_pos().to_owned();
    loop {
        *cursor += 1;
        if *cursor >= elements.len() {
            return if opening == TokenType::Null {
                Err(ZyxtError::from_pos_and_raw(&paren_pos, &elements[*cursor].get_raw())
                    .error_2_1_0(elements[*cursor].get_raw()))
            } else {
                Err(ZyxtError::from_pos_and_raw(&paren_pos, &opening_char.to_string())
                    .error_2_0_1(opening_char.to_string()))
            }
        }
        let catcher_selected = &elements[*cursor];
        if let Element::Token(catcher_selected) = catcher_selected {
            if catcher_selected.type_ == closing && paren_level == 0 {break;}
            else if catcher_selected.type_ == closing {paren_level -= 1;}
            else if catcher_selected.type_ == opening {paren_level += 1;}
        }
        catcher.push(catcher_selected.to_owned())
    }
    Ok(catcher)
}

fn base_split<T: Clone>(parser_fn: &dyn Fn(Vec<Element>) -> Result<T, ZyxtError>, default_val: Option<T>,
                        divider: TokenType, opening: TokenType, closing: TokenType,
                        elements: Vec<Element>, ignore_empty: bool) -> Result<Vec<T>, ZyxtError> {
    let mut out: Vec<T> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut paren_level = 0;
    for element in elements {
        if let Element::Token(Token{type_, ..}) = element {
            if type_ == divider && paren_level == 0 {
                if !ignore_empty && catcher.is_empty() {
                    todo!()
                } else if catcher.is_empty() {
                    out.push(default_val.to_owned().unwrap())
                } else if !catcher.is_empty() {
                    out.push(parser_fn(catcher.to_owned())?);
                }
                catcher.clear();
            } else {
                if type_ == opening {paren_level += 1;}
                else if type_ == closing {paren_level -= 1;}
                catcher.push(element.to_owned());
            }
        } else {catcher.push(element.to_owned());}
    }
    if paren_level != 0 {
        return Err(ZyxtError::from_pos_and_raw(&Default::default(), &"".to_string())
            .error_2_0_1(match opening {
            TokenType::OpenParen => '(',
            TokenType::OpenSquareParen => '[',
            TokenType::OpenCurlyParen => '{',
            TokenType::OpenAngleBracket => '<',
            _ => '?'
        }.to_string())) // TODO
    }
    if out.is_empty() && catcher.is_empty() {return Ok(vec![]);}
    if !ignore_empty && catcher.is_empty() {
        todo!()
    } else if catcher.is_empty() {
        out.push(default_val.unwrap())
    } else if !catcher.is_empty() {
        out.push(parser_fn(catcher.to_owned())?);
    }
    Ok(out)
}

fn split_between(divider: TokenType, opening: TokenType, closing: TokenType,
                 elements: Vec<Element>, ignore_empty: bool) -> Result<Vec<Element>, ZyxtError> {
    base_split(&parse_expr, Some(Element::NullElement), divider, opening, closing,
               elements, ignore_empty)
}

fn get_arguments(cursor: &mut usize, elements: &[Element], raw: &mut String) -> Result<Vec<Argument>, ZyxtError> {
    let contents = catch_between(TokenType::Bar, TokenType::Bar, elements, cursor)?;
    *raw = format!("{}{}{}", raw, contents.iter()
        .map(|e| e.get_raw())
        .collect::<Vec<String>>().join(""), elements[*cursor].get_raw());
    base_split(&|raw_arg| {
        let parts = split_between(TokenType::Colon, TokenType::Null, TokenType::Null,
                                  raw_arg.to_owned(), true)?;
        let name = if let Some(Element::Variable{name, ..}) = parts.get(0) {name.to_owned()} else {
            return Err(ZyxtError::from_pos_and_raw(parts.get(0).unwrap().get_pos(), &raw_arg.get_raw())
                .error_2_1_15(",".to_string()))
        };
        let type_ = if let Some(t) = parts.get(1) {t.to_owned()} else {Element::NullElement};
        let default = parts.get(2).cloned();
        if parts.len() > 3 {
            return Err(ZyxtError::from_pos_and_raw(parts.get(3).unwrap().get_pos(), &raw_arg.get_raw())
                .error_2_1_14(parts[3].get_raw()))
        }
        Ok(Argument{
            name,
            type_: if type_ == Element::NullElement { Type::any()}
            else {type_.as_type()},
            default})
    }, None, TokenType::Comma,
               TokenType::Bar, TokenType::Bar,
               contents, false)
}

fn parse_parens(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            if selected.type_ == TokenType::OpenParen {
                let mut prev_element = &Element::Token(Token{..Default::default()});
                if cursor != 0 { prev_element = &elements[cursor - 1]; }
                if let Element::Token(prev_element) = prev_element { // if selected is Token and is (
                    if cursor == 0 || !prev_element.categories.contains(&TokenCategory::ValueEnd) {
                        let paren_contents = catch_between(TokenType::OpenParen,
                                                           TokenType::CloseParen,
                                                           &elements, &mut cursor)?;
                        new_elements.push(parse_expr(paren_contents)?);
                    } else {new_elements.push(Element::Token(selected.to_owned()))} // or else it's function args
                } else {new_elements.push(Element::Token(selected.to_owned()))}
            } else if selected.type_ == TokenType::OpenCurlyParen { // blocks, {
                let raw = selected.get_raw();
                let paren_contents = catch_between(TokenType::OpenCurlyParen,
                                                   TokenType::CloseCurlyParen,
                                                   &elements, &mut cursor)?;
                new_elements.push(Element::Block {
                    position: selected.position.to_owned(),
                    raw: format!("{}{}{}", raw, paren_contents.iter()
                        .map(|e| e.get_raw())
                        .collect::<Vec<String>>().join(""),
                        elements[cursor].get_raw()),
                    content: parse_block(paren_contents)?
                });
            } else {new_elements.push(Element::Token(selected.to_owned()))}
        } else {new_elements.push(selected.to_owned())}
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
        if let Element::Token(Token{type_: TokenType::Keyword(Keyword::Pre), position, value, ..}) |
               Element::Token(Token{type_: TokenType::Keyword(Keyword::Defer), position, value, ..})= selected {
            if cursor == elements.len()-1 {
                return Err(ZyxtError::from_pos_and_raw(position, value).error_2_1_16())
            }
            let raw = selected.get_raw();
            let is_pre = raw.trim() == "pre";
            check_and_update_cursor!(cursor, selected, elements);
            if let Element::Block {content, raw: block_raw, ..} = selected {
                if is_pre {
                    new_elements.push(Element::Preprocess {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, block_raw),
                        content: content.to_owned()
                    })
                } else {
                    new_elements.push(Element::Defer {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, block_raw),
                        content: content.to_owned()
                    })
                }
            } else {
                let content = parse_expr(elements[cursor..].to_vec())?;
                if is_pre {
                    new_elements.push(Element::Preprocess {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, content.get_raw()),
                        content: vec![content]
                    });
                } else {
                    new_elements.push(Element::Defer {
                        position: position.to_owned(),
                        raw: format!("{}{}", raw, content.get_raw()),
                        content: vec![content]
                    })
                }
                break
            }
        } else {new_elements.push(selected.to_owned())}
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
        if let Element::Token(Token{type_: TokenType::Keyword(keyword), position, ..}) = selected {
        if [Keyword::Class, Keyword::Struct].contains(keyword) {
            let mut raw = selected.get_raw();
            check_and_update_cursor!(cursor, selected, elements);

            let mut args = None;
            if let Element::Token(Token{type_: TokenType::Bar, position, value, ..}) = selected {
                if keyword == &Keyword::Class {
                    return Err(ZyxtError::from_pos_and_raw(
                        position, &format!("class {}", value.trim())).error_2_1_17())
                }
                args = Some(get_arguments(&mut cursor, &elements, &mut raw)?);
                check_and_update_cursor!(cursor, selected, elements);
            }
            let mut content = vec![];
            if let Element::Block {content: block_content, raw: block_raw, ..} = selected {
                content = block_content.to_owned();
                raw = format!("{}{}", raw, block_raw);
            } else if keyword == &Keyword::Class {
                return Err(ZyxtError::from_pos_and_raw(
                    selected.get_pos(), &format!("{}{}", raw, &selected.get_raw())).error_2_1_18(keyword))
            }
            new_elements.push(Element::Class {
                position: position.to_owned(),
                raw,
                is_struct: keyword == &Keyword::Struct,
                class_attrs: Default::default(),
                inst_attrs: Default::default(),
                content,
                args,
            })
        } else {new_elements.push(selected.to_owned())}} else {new_elements.push(selected.to_owned())}
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
        if let Element::Token(selected) = selected { match selected.type_ {
            TokenType::DotOpr => { // TODO rewrite this
                if cursor == 0 {
                    return Err(ZyxtError::from_element(&elements[cursor])
                        .error_2_1_0(String::from("."))) // could be enum but thats for later
                } else if cursor == elements.len()-1 {
                    return Err(ZyxtError::from_element(&elements[cursor])
                        .error_2_1_2()) // definitely at the wrong place
                }
                let prev_element = &elements[cursor-1];
                let next_element = &elements[cursor+1];
                if let Element::Token(next_element) = next_element {
                    if next_element.type_ != TokenType::Variable {
                        return Err(ZyxtError::from_token(next_element).error_2_1_0(next_element.value.to_owned()))
                    }} else if let Element::Literal{content, ..} = next_element {
                    return Err(ZyxtError::from_element(next_element).error_2_1_0(content.to_owned()))
                }
                if let (Element::Token(prev_element), Element::Token(next_element)) = (prev_element, next_element) {
                    if !prev_element.categories.contains(&TokenCategory::ValueEnd) {
                        return Err(ZyxtError::from_token(selected).error_2_1_0(String::from("."))) //could be enum but thats for later
                    }
                    catcher = Element::Variable{
                        position: next_element.position.to_owned(),
                        name: next_element.value.to_owned(),
                        raw: format!("{}{}{}", catcher.get_raw(), selected.get_raw(), next_element.get_raw()),
                        parent: Box::new(catcher)
                    };
                    cursor += 1;
                } else if let Element::Token(next_element) = next_element {
                    catcher = Element::Variable{
                        position: next_element.position.to_owned(),
                        name: next_element.value.to_owned(),
                        raw: format!("{}{}{}", catcher.get_raw(), selected.get_raw(), next_element.get_raw()),
                        parent: Box::new(catcher)
                    };
                } else {
                    return Err(ZyxtError::from_token(selected).error_2_1_0(String::from("."))) // definitely at the wrong place
                }

            }
            TokenType::Variable => {
                if catcher != Element::NullElement {new_elements.push(catcher.to_owned());}
                catcher = Element::Variable {
                    position: selected.position.to_owned(),
                    name: selected.value.to_owned(),
                    raw: selected.get_raw(),
                    parent: Box::new(Element::NullElement)
                }
            }
            TokenType::LiteralNumber |
            TokenType::LiteralMisc |
            TokenType::LiteralString => {
                if catcher != Element::NullElement {new_elements.push(catcher.to_owned());}
                catcher = Element::Literal {
                    position: selected.position.to_owned(),
                    raw: selected.get_raw(),
                    type_: Type::from_str(if selected.type_ == TokenType::LiteralMisc {
                        match &*selected.value {
                            "true" | "false" => "bool",
                            "null" => "_null",
                            "inf" | "undef" => "_num",
                            _ => unreachable!("{}", selected.value)
                        }
                    } else if selected.type_ == TokenType::LiteralNumber{
                        if selected.value.contains('.') {"f64"}
                        else if selected.value.parse::<i32>().is_ok() {"i32"}
                        else if selected.value.parse::<i64>().is_ok() {"i64"}
                        else if selected.value.parse::<i128>().is_ok() {"i128"}
                        else if selected.value.parse::<u128>().is_ok() {"u128"}
                        else {"ibig"}
                    } else {"str"}),
                    content: if selected.type_ == TokenType::LiteralString {
                        selected.value[1..selected.value.len()-1].to_string()
                    } else {selected.value.to_owned()}
                }
            }
            TokenType::CloseParen => {
                return Err(ZyxtError::from_token(selected).error_2_0_2(')'.to_string()))
            }
            TokenType::OpenParen => {
                if cursor == 0 {
                    return Err(ZyxtError::from_token(selected).error_2_1_0(String::from("("))) // parens should have been settled in the first part
                }
                let mut raw = selected.get_raw();
                let contents = catch_between(TokenType::OpenParen,
                                             TokenType::CloseParen,
                                             &elements, &mut cursor)?;
                raw = format!("{}{}{}", raw, contents.iter()
                    .map(|e| e.get_raw())
                    .collect::<Vec<String>>().join(""),
                    elements[cursor].get_raw());
                let args = split_between(TokenType::Comma,
                                                       TokenType::OpenParen, TokenType::CloseParen,
                                                       contents,
                                                       false)?;
                catcher = Element::Call {
                    position: selected.position.to_owned(),
                    raw: format!("{}{}", catcher.get_raw(), raw),
                    called: Box::new(catcher),
                    args, kwargs: HashMap::new()
                }
            }
            _ => {
                if catcher != Element::NullElement {new_elements.push(catcher.to_owned());}
                catcher = Element::NullElement;
                new_elements.push(Element::Token(selected.to_owned()));
            }
        }} else {
            if catcher != Element::NullElement {new_elements.push(catcher.to_owned());}
            catcher = selected.to_owned()
        }
        cursor += 1;
    }
    if catcher != Element::NullElement {new_elements.push(catcher);}
    //catcher2 = Element::NullElement;
    Ok(new_elements)
}

fn parse_procs_and_fns(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];
    let mut cursor = 0;
    let mut selected;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_, ..}) = selected {
            if [TokenType::Keyword(Keyword::Proc),
                TokenType::Keyword(Keyword::Fn),
                TokenType::Bar].contains(type_) {
                let position = selected.get_pos().to_owned();
                let is_fn= if type_ != &TokenType::Bar {
                    type_ == &TokenType::Keyword(Keyword::Fn)
                } else {false};
                let mut raw = selected.get_raw().to_owned();
                if type_ != &TokenType::Bar {
                    check_and_update_cursor!(cursor, selected, elements);
                    raw = format!("{}{}", raw, selected.get_raw());
                }

                let args = if let Element::Token(Token{type_: TokenType::Bar, ..}) = selected {
                    get_arguments(&mut cursor, &elements, &mut raw)?
                } else {cursor -= 1; vec![]};

                check_and_update_cursor!(cursor, selected, elements);
                let return_type = if let Element::Token(Token{type_: TokenType::Colon, value, ..}) = selected {
                    let mut catcher = vec![];
                    raw = format!("{}{}", raw, value);
                    loop {
                        check_and_update_cursor!(cursor, selected, elements);
                        raw = format!("{}{}", raw, selected.get_raw());
                        if let Element::Block{..} = selected {break;}
                        catcher.push(selected.to_owned());
                    }
                    if let Element::Variable {name, ..} = parse_expr(catcher)? {
                        Type::Instance {
                            name,
                            type_args: vec![],
                            inst_attrs: Default::default(),
                            implementation: None
                        }
                    } else {todo!("throw error here")}
                } else {Type::null()};

                if let Element::Block{content, ..} = selected {
                    new_elements.push(Element::Procedure {
                        position, is_fn, args,
                        return_type,
                        raw,
                        content: content.to_owned()
                    });
                } else {
                    let content = parse_expr(elements[cursor..].to_vec())?;
                    new_elements.push(Element::Procedure {
                        position, is_fn, args,
                        return_type,
                        raw: format!("{}{}", raw, content.get_raw()),
                        content: vec![content]
                    });
                    return Ok(new_elements);
                }

            } else {new_elements.push(selected.to_owned())}} else {new_elements.push(selected.to_owned())}
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_assignment_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {return Ok(vec![])}
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::AssignmentOpr(opr_type), position, ..}) = ele {
            if i == 0 || i == elements.len()-1 {
                return Err(ZyxtError::from_element(ele).error_2_1_3(ele.get_raw()))
            }
            let variable = parse_expr(vec![elements[i-1].to_owned()])?;
            let content = if opr_type == &OprType::Null {
                parse_expr(elements[i+1..].to_vec())?
            } else {
                let operand2 = parse_expr(elements[i+1..].to_vec())?;
                Element::BinaryOpr {
                    position: position.to_owned(),
                    type_: *opr_type,
                    raw: operand2.get_raw(),
                    operand1: Box::new(variable.to_owned()),
                    operand2: Box::new(operand2)
                }
            };

            return Ok(elements[..i-1].iter().cloned()
                .chain(vec![Element::Set {
                    position: position.to_owned(),
                    raw: format!("{}{}{}", variable.get_raw(), ele.get_raw(), content.get_raw()),
                    variable: Box::new(variable),
                    content: Box::new(content)
                }]).collect::<Vec<Element>>())
        }
    }
    Ok(elements)
}

fn parse_un_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {return Ok(vec![])}
    for (i, ele) in elements.iter().enumerate().rev() {
        if let Element::Token(Token{type_: TokenType::UnaryOpr(opr_type, opr_side), position, ..}) = ele {
            if opr_side == &Side::Left {
                if i == elements.len()-1 {
                    return Err(ZyxtError::from_element(ele).error_2_1_4(ele.get_raw()))
                }
                let operand = parse_un_oprs(elements[i+1..].to_vec())?[0].to_owned();
                return parse_un_oprs(elements[..i].iter().cloned()
                    .chain(vec![Element::UnaryOpr {
                        position: position.to_owned(),
                        type_: *opr_type,
                        raw: format!("{}{}", ele.get_raw(), operand.get_raw()),
                        operand: Box::new(operand)
                    }]).collect::<Vec<Element>>())
            } else if opr_side == &Side::Right {
                if i == 0 {
                    return Err(ZyxtError::from_element(ele).error_2_1_4(ele.get_raw()))
                }
                let operand = parse_un_oprs(elements[..i].to_vec())?[0].to_owned();
                return parse_un_oprs(vec![Element::UnaryOpr {
                    position: position.to_owned(),
                    type_: *opr_type,
                    raw: format!("{}{}", operand.get_raw(), ele.get_raw()),
                    operand: Box::new(operand)
                }].into_iter()
                    .chain(elements[i+1..].to_vec())
                    .collect::<Vec<Element>>());
            }
        }
    }
    Ok(elements)
}

fn parse_normal_oprs(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    if elements.is_empty() {return Ok(vec![])}
    let mut highest_order_index: usize = 0;
    let mut highest_order = 0;
    let mut opr_detected = false;
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), value, .. }) = ele {
            if i == 0 || i == elements.len()-1 {
                return Err(ZyxtError::from_element(ele).error_2_1_3(value.to_owned()))
            }
            if get_order(opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(opr_type);
                opr_detected = true
            }}
    }
    Ok(if !opr_detected {elements}
    else if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, ..}) = &elements[highest_order_index] {
        let operand1 = parse_expr(elements[..highest_order_index].to_vec())?;
        let operand2 = parse_expr(elements[highest_order_index+1..].to_vec())?;
        vec![Element::BinaryOpr {
            position: position.to_owned(),
            type_: *opr_type,
            raw: format!("{}{}{}", operand1.get_raw(), elements[highest_order_index].get_raw(), operand2.get_raw()),
            operand1: Box::new(operand1),
            operand2: Box::new(operand2)
        }]
    } else {elements})
}

fn parse_delete_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::Keyword(Keyword::Delete), ..}) = ele {
            let vars_to_delete = split_between(TokenType::Comma,
                                               TokenType::Null, TokenType::Null,
                                               elements[i+1..].to_vec(), false)?;
            let mut varnames = vec![];
            for var in vars_to_delete.iter() {
                if let Element::Variable {name, ..} = var {
                    varnames.push(name.to_owned());
                }
                else if let Element::UnaryOpr {type_: OprType::Deref, raw, ..} = var {
                    return Err(ZyxtError::from_element(var).error_2_1_12(raw.to_owned()))
                }
                else {
                    return Err(ZyxtError::from_element(var).error_2_1_11(var.get_raw()))
                }
            }
            new_elements.push(Element::Delete {
                position: ele.get_pos().to_owned(),
                raw: format!("{}{}", ele.get_raw(), elements[i+1..].iter()
                    .map(|e| e.get_raw())
                    .collect::<Vec<String>>().join("")),
                names: varnames
            });
            return Ok(new_elements)
        }
        new_elements.push(ele.to_owned());
    }
    Ok(elements)
}

fn parse_return_expr(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token { type_: TokenType::Keyword(Keyword::Return), whitespace, value, .. }) = ele {
            let return_val = parse_expr(elements[i+1..].to_vec())?;
            new_elements.push(Element::Return {
                position: ele.get_pos().to_owned(),
                raw: format!("{}{}{}", whitespace, value, return_val.get_raw()),
                value: Box::new(return_val)
            });
            return Ok(new_elements)
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
        if let Element::Token(Token{type_: TokenType::Flag(_), ..}) = selected {flag_pos = Some(cursor);}
        if let Element::Token(Token{type_: TokenType::DeclarationOpr, position,
                                  whitespace, value, ..}) = selected {
            if cursor == elements.len() - 1 || cursor == 0 {
                return Err(ZyxtError::from_element(selected).error_2_1_5())
            }
            let declared_var: &Element = &elements[cursor-1];
            let mut raw = format!("{}{}{}", declared_var.get_raw(), whitespace, value);
            let flags = if flag_pos == None {vec![]} else {
                let mut f = vec![];
                for ele in elements[flag_pos.unwrap()..cursor-1].iter() {
                    if let Element::Token(Token{type_: TokenType::Flag(flag), whitespace, value, ..}) = &ele {
                        raw = format!("{}{}{}", whitespace, value, raw);
                        f.push(*flag);
                    } else {
                        return Err(ZyxtError::from_element(ele)
                            .error_2_1_6(ele.get_raw()))
                    }
                }
                f
            };
            for _ in 0..flags.len()+1 {new_elements.pop();}
            let content = parse_expr(elements[cursor+1..].to_vec())?;
            new_elements.push(Element::Declare {
                position: position.to_owned(),
                raw: format!("{}{}", raw, content.get_raw()),
                variable: Box::new(parse_expr(vec![declared_var.to_owned()])?),
                content: Box::new(content),
                flags,
                type_: Type::null() // TODO type later
            });
            break;
        } else {new_elements.push(selected.to_owned())}
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
        if let Element::Token(Token{type_: TokenType::Keyword(kwd),
                                  position, ..}) = selected { match kwd {
            Keyword::If => {
                let start_pos = position.to_owned();
                let mut conditions: Vec<Condition> = vec![];
                let mut prev_catcher_kwd = "";
                let mut raw = String::new();
                loop {
                    let catcher_kwd;
                    let mut catcher_selected = &elements[cursor];
                    if let Element::Token(Token{type_: TokenType::Keyword(prekwd), position,
                                              whitespace, value, ..}) = catcher_selected {
                        catcher_kwd = match prekwd {
                            Keyword::If if position == &start_pos => "if",
                            Keyword::Elif if prev_catcher_kwd != "else" => "elif",
                            Keyword::Else if prev_catcher_kwd != "else" => "else",
                            Keyword::Elif if prev_catcher_kwd == "else" => {
                                return Err(ZyxtError::from_element(catcher_selected).error_2_1_7("elif".to_string()))
                            },
                            Keyword::Else if prev_catcher_kwd == "else" => {
                                return Err(ZyxtError::from_element(catcher_selected).error_2_1_7("else".to_string()))
                            },
                            _ => break
                        };
                        raw = format!("{}{}{}", raw, whitespace, value);
                    } else {break}
                    prev_catcher_kwd = catcher_kwd;
                    check_and_update_cursor!(cursor, catcher_selected, elements);
                    let condition= if catcher_kwd == "else" {
                        Element::NullElement
                    } else if let Element::Block {raw: block_raw, ..} = catcher_selected {
                        raw = format!("{}{}", raw, block_raw);
                        check_and_update_cursor!(cursor, catcher_selected, elements);
                        catcher_selected.to_owned()
                    } else {
                        let mut catcher = vec![elements[cursor].to_owned()];
                        loop {
                            check_and_update_cursor!(cursor, catcher_selected, elements);
                            raw = format!("{}{}", raw, catcher_selected.get_raw());
                            if let Element::Block {..} = catcher_selected {break}
                            else {catcher.push(catcher_selected.to_owned());}
                        };
                        parse_expr(catcher)?
                    };
                    catcher_selected = &elements[cursor];
                    raw = format!("{}{}", raw, catcher_selected.get_raw());
                    if let Element::Block {content, ..} = catcher_selected {
                        conditions.push(Condition {
                            condition,
                            if_true: content.to_owned()
                        })
                    } else {
                        return Err(ZyxtError::from_element(selected).error_2_1_8(catcher_selected.get_raw()))
                    }
                    cursor += 1;
                    if cursor == elements.len() {break;}
                }
                new_elements.push(Element::If {
                    position: start_pos,
                    raw,
                    conditions
                });
                cursor -= 1;
            },
            Keyword::Elif | Keyword::Else => {
                return Err(ZyxtError::from_element(selected)
                    .error_2_1_9(if kwd == &Keyword::Elif {"elif"} else {"else"}.to_string()))
            },
            _ => new_elements.push(selected.to_owned())
        }} else {new_elements.push(selected.to_owned());}
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_unparen_calls(elements: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    let comma_pos = elements.iter()
        .position(|e| matches!(e, Element::Token(Token{type_: TokenType::Comma, ..})))
        .unwrap_or(elements.len());
    let right_un_pos = elements.iter().enumerate()
        .take_while(|(i, _)| *i < comma_pos)
        .collect::<Vec<_>>().iter()
        .rposition(|(_, e)| matches!(e, Element::Token(Token{type_: TokenType::UnaryOpr(_, Side::Right), ..})));
    if let Some(right_un_pos) = right_un_pos {
        if right_un_pos + 1 != comma_pos {
            let min_index = min(right_un_pos+1, elements.len());
            return parse_unparen_calls(parse_un_oprs(elements[..min_index].to_vec())?
                .into_iter()
                .chain(elements[min_index..].iter().cloned())
                .collect())
        }
    }
    let left_un_pos = elements.iter().enumerate()
        .take_while(|(i, _)| *i < comma_pos)
        .collect::<Vec<_>>().iter()
        .rposition(|(_, e)| matches!(e, Element::Token(Token{type_: TokenType::UnaryOpr(_, Side::Left), ..})));
    if let Some(left_un_pos) = left_un_pos {
        if left_un_pos < comma_pos {
            let min_index = min(left_un_pos+2, elements.len());
            return parse_unparen_calls(parse_un_oprs(elements[..min_index].to_vec())?
                .into_iter()
                .chain(elements[min_index..].iter().cloned())
                .collect())
        }
    }

    if elements.len() == 1 {return Ok(elements)}
    Ok(vec![Element::Call {
        position: elements[0].get_pos().to_owned(),
        raw: elements.iter()
            .map(|e| e.get_raw())
            .collect::<Vec<String>>().join(""),
        called: Box::new(elements[0].to_owned()),
        args: split_between(TokenType::Comma,
                            TokenType::Null, TokenType::Null,
                            elements[1..].to_vec(),
                            false)?,
        kwargs: Default::default()
    }])
}

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
        return Err(ZyxtError::from_element(&elements[1]).error_2_1_0(elements[1].get_raw()))
    }
    Ok(elements.get(0).unwrap_or(&Element::NullElement).to_owned())
}

fn parse_block(input: Vec<Element>) -> Result<Vec<Element>, ZyxtError> {
    split_between(TokenType::StatementEnd,
                  TokenType::OpenCurlyParen,
                  TokenType::CloseCurlyParen,
                  input, true)
}

pub fn parse_token_list(mut input: Vec<Token>) -> Result<Vec<Element>, ZyxtError> {
    let mut comments: Vec<Element> = vec![];

    // detect & remove comments
    for token in input.iter() {
        if token.type_ == TokenType::Comment {
            comments.push(Element::Comment {
                position: token.position.to_owned(),
                raw: token.get_raw(),
                content: token.value.to_owned()
            })
        } else if [TokenType::CommentStart,
            TokenType::CommentEnd,
            TokenType::MultilineCommentStart,
            TokenType::MultilineCommentEnd].contains(&token.type_) {
            return Err(ZyxtError::from_token(token).error_2_1_10(token.value.to_owned()))
        }
    }

    input = input.into_iter().filter(|token| token.type_ != TokenType::Comment).collect();

    // generate and return an AST for each expression
    parse_block(input.into_iter().map(Element::Token)
                    .collect::<Vec<Element>>())
}