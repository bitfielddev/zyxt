use std::collections::HashMap;
use crate::objects::token::{TokenCategory, TokenType, get_order, Side, OprType, Keyword};
use crate::objects::element::{Argument, Condition, Element};
use crate::{Token, ZyxtError};
use crate::objects::position::Position;
use crate::objects::typeobj::TypeObj;

fn catch_between(opening: TokenType, closing: TokenType,
                 elements: &Vec<Element>, cursor: &mut usize) -> Result<Vec<Element>, ZyxtError> {
    let mut paren_level = 0;
    let mut catcher: Vec<Element> = vec![];
    let opening_char = match opening {
        TokenType::OpenParen => '(',
        TokenType::OpenSquareParen => '[',
        TokenType::OpenCurlyParen => '{',
        TokenType::OpenAngleBracket => '<',
        _ => '?'
    };
    let paren_pos = elements[*cursor].get_pos().clone();
    loop {
        *cursor += 1;
        if *cursor >= elements.len() {
            return if opening == TokenType::Null {
                Err(ZyxtError::from_pos(&paren_pos).error_2_1_0("TODO".to_string()))
            } else {
                Err(ZyxtError::from_pos(&paren_pos).error_2_0_1(opening_char))
            }
        }
        let catcher_selected = &elements[*cursor];
        if let Element::Token(catcher_selected) = catcher_selected {
            if catcher_selected.type_ == closing && paren_level == 0 {break;}
            else if catcher_selected.type_ == closing {paren_level -= 1;}
            else if catcher_selected.type_ == opening {paren_level += 1;}
        }
        catcher.push(catcher_selected.clone())
    }
    Ok(catcher)
}

fn base_split<T: Clone>(parser_fn: &dyn Fn(Vec<Element>, &String) -> Result<T, ZyxtError>, default_val: Option<T>,
                        divider: TokenType, opening: TokenType, closing: TokenType,
                        elements: Vec<Element>, filename: &String, ignore_empty: bool) -> Result<Vec<T>, ZyxtError> {
    let mut out: Vec<T> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut paren_level = 0;
    for element in elements {
        if let Element::Token(Token{type_, ..}) = element {
            if type_ == divider && paren_level == 0 {
                if !ignore_empty && catcher.len() == 0 {
                    todo!()
                } else if catcher.len() == 0 {
                    out.push(default_val.clone().unwrap())
                } else if catcher.len() != 0 {
                    out.push(parser_fn(catcher.clone(), filename)?);
                }
                catcher.clear();
            } else {
                if type_ == opening {paren_level += 1;}
                else if type_ == closing {paren_level -= 1;}
                catcher.push(element.clone());
            }
        } else {catcher.push(element.clone());}
    }
    if paren_level != 0 {
        return Err(ZyxtError::from_pos(&Default::default()).error_2_0_1(match opening {
            TokenType::OpenParen => '(',
            TokenType::OpenSquareParen => '[',
            TokenType::OpenCurlyParen => '{',
            TokenType::OpenAngleBracket => '<',
            _ => '?'
        })) // TODO
    }
    if out.len() == 0 && catcher.len() == 0 {return Ok(vec![]);}
    if !ignore_empty && catcher.len() == 0 {
        todo!()
    } else if catcher.len() == 0 {
        out.push(default_val.unwrap())
    } else if catcher.len() != 0 {
        out.push(parser_fn(catcher.clone(), filename)?);
    }
    Ok(out)
}

fn split_between(divider: TokenType, opening: TokenType, closing: TokenType,
                 elements: Vec<Element>, filename: &String, ignore_empty: bool) -> Result<Vec<Element>, ZyxtError> {
    base_split(&parse_expr, Some(Element::NullElement), divider, opening, closing,
               elements, filename, ignore_empty)
}

fn parse_parens(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
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
                    if cursor == 0 && !prev_element.categories.contains(&TokenCategory::ValueEnd) {
                        let paren_contents = catch_between(TokenType::OpenParen,
                                                           TokenType::CloseParen,
                                                           &elements, &mut cursor)?;
                        new_elements.push(parse_expr(paren_contents, &filename)?);
                    } else {new_elements.push(Element::Token(selected.clone()))} // or else it's function args
                } else {new_elements.push(Element::Token(selected.clone()))}
            } else if selected.type_ == TokenType::OpenCurlyParen { // blocks, {
                let paren_contents = catch_between(TokenType::OpenCurlyParen,
                                                   TokenType::CloseCurlyParen,
                                                   &elements, &mut cursor)?;
                new_elements.push(Element::Block {
                    position: selected.position.clone(),
                    content: parse_block(paren_contents, filename)?
                });
            } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }

    Ok(new_elements)
}

fn parse_vars_literals_and_calls(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut catcher: Element = Element::NullElement;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected { match selected.type_ {
            TokenType::DotOpr => { // TODO rewrite this
                if cursor == 0 {
                    return Err(ZyxtError::from_pos(&selected.position).error_2_1_0(String::from("."))) // could be enum but thats for later
                } else if cursor == elements.len()-1 {
                    return Err(ZyxtError::from_pos(&selected.position).error_2_1_2()) // definitely at the wrong place
                }
                let prev_element = &elements[cursor-1];
                let next_element = &elements[cursor+1];
                if let Element::Token(next_element) = next_element {
                    if next_element.type_ != TokenType::Variable {
                        return Err(ZyxtError::from_pos(&next_element.position).error_2_1_0(next_element.value.clone()))
                    }} else if let Element::Literal{position, content, ..} = next_element {
                    return Err(ZyxtError::from_pos(position).error_2_1_0(content.clone()))
                }
                if let (Element::Token(prev_element), Element::Token(next_element)) = (prev_element, next_element) {
                    if !prev_element.categories.contains(&TokenCategory::ValueEnd) {
                        return Err(ZyxtError::from_pos(&selected.position).error_2_1_0(String::from("."))) //could be enum but thats for later
                    }
                    catcher = Element::Variable{
                        position: next_element.position.clone(),
                        name: next_element.value.clone(),
                        parent: Box::new(catcher)
                    };
                    cursor += 1;
                } else if let Element::Token(next_element) = next_element {
                    catcher = Element::Variable{
                        position: next_element.position.clone(),
                        name: next_element.value.clone(),
                        parent: Box::new(catcher)
                    };
                } else {
                    return Err(ZyxtError::from_pos(&selected.position).error_2_1_0(String::from("."))) // definitely at the wrong place
                }

            }
            TokenType::Variable => {
                if catcher != Element::NullElement {new_elements.push(catcher.clone());}
                catcher = Element::Variable {
                    position: selected.position.clone(),
                    name: selected.value.clone(),
                    parent: Box::new(Element::NullElement)
                }
            }
            TokenType::LiteralNumber |
            TokenType::LiteralMisc |
            TokenType::LiteralString => {
                if catcher != Element::NullElement {new_elements.push(catcher.clone());}
                catcher = Element::Literal {
                    position: selected.position.clone(),
                    type_: TypeObj::from_str(if selected.type_ == TokenType::LiteralMisc {
                        match &*selected.value {
                            "true" | "false" => "bool",
                            "null" => "#null",
                            "inf" | "undef" => "#num",
                            _ => panic!("{}", selected.value)
                        }
                    } else if selected.type_ == TokenType::LiteralNumber{
                        if selected.value.contains(".") {"f64"} else {"i32"}
                    } else {"str"}),
                    content: selected.value.clone()
                }
            }
            TokenType::CloseParen => {
                return Err(ZyxtError::from_pos(&selected.position).error_2_0_2(')'))
            }
            TokenType::OpenParen => {
                if cursor == 0 {
                    return Err(ZyxtError::from_pos(&selected.position).error_2_1_0(String::from("("))) // parens should have been settled in the first part
                }
                let args = split_between(TokenType::Comma,
                                                       TokenType::OpenParen, TokenType::CloseParen,
                                                       catch_between(TokenType::OpenParen,
                                                                     TokenType::CloseParen,
                                                                     &elements, &mut cursor)?,
                                                       filename, false)?;
                catcher = Element::Call {
                    position: selected.position.clone(),
                    called: Box::new(catcher),
                    args, kwargs: Box::new(HashMap::new())
                }
            }
            _ => {
                if catcher != Element::NullElement {new_elements.push(catcher.clone());}
                catcher = Element::NullElement;
                new_elements.push(Element::Token(selected.clone()));
            }
        }} else {
            if catcher != Element::NullElement {new_elements.push(catcher.clone());}
            catcher = selected.clone()
        }
        cursor += 1;
    }
    if catcher != Element::NullElement {new_elements.push(catcher);}
    //catcher2 = Element::NullElement;
    Ok(new_elements)
}

fn parse_procs_and_fns(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];
    let mut cursor = 0;
    let mut selected;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_, ..}) = selected {
            if [TokenType::Keyword(Keyword::Proc),
                TokenType::Keyword(Keyword::Fn),
                TokenType::Bar].contains(type_) {
                let position = selected.get_pos().clone();
                let is_fn= if type_ != &TokenType::Bar {
                    type_ == &TokenType::Keyword(Keyword::Fn)
                } else {false};
                if type_ != &TokenType::Bar {
                    cursor += 1;
                    if cursor >= elements.len() {
                        return Err(ZyxtError::from_pos(selected.get_pos()).error_2_1_0("TODO".to_string()))
                    }
                    selected = &elements[cursor];
                }

                let args = if let Element::Token(Token{type_: TokenType::Bar, ..}) = selected {
                    base_split(&|raw_arg, filename| {
                        let parts = split_between(TokenType::Colon, TokenType::Null, TokenType::Null,
                                                  raw_arg, filename, true)?;
                        let name = if let Some(Element::Variable{name, ..}) = parts.get(0) {name.clone()} else {
                            return Err(ZyxtError::from_pos(parts.get(0).unwrap().get_pos()).error_2_1_15("TODO".to_string()))
                        };
                        let type_ = if let Some(t) = parts.get(1) {t.clone()} else {Element::NullElement};
                        let default = if let Some(d) = parts.get(2) {Some(d.clone())} else {None};
                        if parts.len() > 3 {
                            return Err(ZyxtError::from_pos(parts.get(3).unwrap().get_pos()).error_2_1_14("TODO".to_string()))
                        }
                        Ok(Argument{
                            name,
                            type_: if type_ == Element::NullElement {TypeObj::any()}
                            else {TypeObj::Compound(Box::new(type_))},
                            default})
                    }, None, TokenType::Comma,
                               TokenType::Bar, TokenType::Bar,
                               catch_between(TokenType::Bar, TokenType::Bar, &elements, &mut cursor)?,
                               filename, false)
                } else {cursor -= 1; Ok(vec![])}?;

                cursor += 1;
                if cursor >= elements.len() {
                    return Err(ZyxtError::from_pos(selected.get_pos()).error_2_1_0("TODO".to_string()))
                }
                selected = &elements[cursor];
                let return_type = if let Element::Token(Token{type_: TokenType::Colon, ..}) = selected {
                    let mut catcher = vec![];
                    loop {
                        cursor += 1;
                        if cursor >= elements.len() {
                            return Err(ZyxtError::from_pos(selected.get_pos()).error_2_1_0("TODO".to_string()))
                        }
                        selected = &elements[cursor];
                        if let Element::Block{..} = selected {break;}
                        catcher.push(selected.clone());
                    }
                    TypeObj::Compound(Box::new(parse_expr(catcher, filename)?))
                } else {TypeObj::null()};

                if let Element::Block{content, ..} = selected {
                    new_elements.push(Element::Procedure {
                        position, is_fn, args,
                        return_type,
                        content: content.clone()
                    });
                } else {
                    new_elements.push(Element::Procedure {
                        position, is_fn, args,
                        return_type,
                        content: vec![parse_expr(elements[cursor..].to_vec(), filename)?]
                    });
                    return Ok(new_elements);
                }

            } else {new_elements.push(selected.clone())}} else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_assignment_oprs(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    if elements.len() == 0 {return Ok(vec![])}
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::AssignmentOpr(opr_type), position, ..}) = ele {
            if i == 0 || i == elements.len()-1 {
                return Err(ZyxtError::from_pos(position).error_2_1_3("TODO".to_string()))
            }
            let variable = parse_expr(vec![elements[i-1].clone()], filename)?;
            let content = if opr_type == &OprType::Null {
                parse_expr(elements[i+1..].to_vec(), filename)?
            } else {
                Element::BinaryOpr {
                    position: position.clone(),
                    type_: opr_type.clone(),
                    operand1: Box::new(variable.clone()),
                    operand2: Box::new(parse_expr(elements[i+1..].to_vec(), filename)?)
                }
            };

            return Ok(elements[..i-1].to_vec().into_iter()
                .chain(vec![Element::Set {
                    position: position.clone(),
                    variable: Box::new(variable),
                    content: Box::new(content)
                }]).collect::<Vec<Element>>())
        }
    }
    Ok(elements)
}

fn parse_un_oprs(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    if elements.len() == 0 {return Ok(vec![])}
    for (i, ele) in elements.iter().enumerate().rev() {
        if let Element::Token(Token{type_: TokenType::UnaryOpr(opr_type, opr_side), position, ..}) = ele {
            if opr_side == &Side::Left {
                if i == elements.len()-1 {
                    return Err(ZyxtError::from_pos(position).error_2_1_4("TODO".to_string()))
                }
                return Ok(elements[..i].to_vec().into_iter()
                    .chain(vec![Element::UnaryOpr {
                        position: position.clone(),
                        type_: *opr_type,
                        operand: Box::new(parse_expr(elements[i+1..].to_vec(), filename)?)
                    }]).collect::<Vec<Element>>())
            } else if opr_side == &Side::Right {
                if i == 0 {
                    return Err(ZyxtError::from_pos(position).error_2_1_4("TODO".to_string()))
                }
                return Ok(vec![Element::UnaryOpr {
                    position: position.clone(),
                    type_: *opr_type,
                    operand: Box::new(parse_expr(elements[..i].to_vec(), filename)?)
                }].into_iter()
                    .chain(elements[i+1..].to_vec())
                    .collect::<Vec<Element>>());
            }
        }
    }
    Ok(elements)
}

fn parse_normal_oprs(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    if elements.len() == 0 {return Ok(vec![])}
    let mut highest_order_index: usize = 0;
    let mut highest_order = 0;
    let mut opr_detected = false;
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, value, .. }) = ele {
            if i == 0 || i == elements.len()-1 {
                return Err(ZyxtError::from_pos(position).error_2_1_3(value.clone()))
            }
            if get_order(&opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(&opr_type);
                opr_detected = true
            }}
    }
    Ok(if !opr_detected {elements}
    else if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, ..}) = &elements[highest_order_index] {
        vec![Element::BinaryOpr {
            position: position.clone(),
            type_: *opr_type,
            operand1: Box::new(parse_expr(elements[..highest_order_index].to_vec(), filename)?),
            operand2: Box::new(parse_expr(elements[highest_order_index+1..].to_vec(), filename)?)
        }]
    } else {elements})
}

fn parse_delete_expr(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::Keyword(Keyword::Delete), ..}) = ele {
            let vars_to_delete = split_between(TokenType::Comma,
                                               TokenType::Null, TokenType::Null,
                                               elements[i+1..].to_vec(), filename, false)?;
            let mut varnames = vec![];
            for var in vars_to_delete.iter() {
                if let Element::Variable {name, ..} = var {
                    varnames.push(name.clone());
                }
                else if let Element::UnaryOpr {type_: OprType::Deref, position, ..} = var {
                    return Err(ZyxtError::from_pos(position).error_2_1_12("TODO".to_string()))
                }
                else {
                    return Err(ZyxtError::from_pos(var.get_pos()).error_2_1_11("TODO".to_string()))
                }
            }
            new_elements.push(Element::Delete {
                position: ele.get_pos().clone(),
                names: varnames
            });
            return Ok(new_elements)
        }
        new_elements.push(ele.clone());
    }
    Ok(elements)
}

fn parse_return_expr(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut new_elements = vec![];

    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token { type_: TokenType::Keyword(Keyword::Return), .. }) = ele {
            let return_val = parse_expr(elements[i+1..].to_vec(), filename)?;
            new_elements.push(Element::Return {
                position: ele.get_pos().clone(),
                value: Box::new(return_val)
            });
            return Ok(new_elements)
        }
        new_elements.push(ele.clone());
    }
    Ok(elements)
}

fn parse_declaration_expr(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    let mut flag_pos = None;
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::Flag(_), ..}) = selected {flag_pos = Some(cursor);}
        if let Element::Token(Token{type_: TokenType::DeclarationOpr, position, ..}) = selected {
            if cursor == elements.len() - 1 || cursor == 0 {
                return Err(ZyxtError::from_pos(position).error_2_1_5())
            }
            let declared_var = &elements[cursor-1];
            let flags = if flag_pos == None {vec![]} else {
                let mut f = vec![];
                for i in flag_pos.unwrap()..cursor-1 {
                    if let Element::Token(Token{type_: TokenType::Flag(flag), ..}) = &elements[i] {
                        f.push(*flag);
                    } else {
                        return Err(ZyxtError::from_pos(&Position{filename: filename.clone(), line: 0, column: 0})
                            .error_2_1_6(String::from(""))) // TODO
                    }
                }
                f
            };
            for _ in 0..flags.len()+1 {new_elements.pop();}
            new_elements.push(Element::Declare {
                position: position.clone(),
                variable: Box::new(parse_expr(vec![declared_var.clone()], filename)?),
                content: Box::new(parse_expr(elements[cursor+1..].to_vec(), filename)?),
                flags,
                type_: TypeObj::null() // TODO type later
            });
            break;
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    Ok(new_elements)
}

pub fn parse_if_expr(elements: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::Keyword(kwd), position, ..}) = selected { match kwd {
            Keyword::If => {
                let start_pos = position.clone();
                let mut conditions: Vec<Condition> = vec![];
                let mut prev_catcher_kwd = "";
                loop {
                    let catcher_kwd;
                    let mut catcher_selected = &elements[cursor];
                    if let Element::Token(Token{type_: TokenType::Keyword(prekwd), position, ..}) = catcher_selected {
                        catcher_kwd = match prekwd {
                            Keyword::If if position == &start_pos => "if",
                            Keyword::Elif if prev_catcher_kwd != "else" => "elif",
                            Keyword::Else if prev_catcher_kwd != "else" => "else",
                            Keyword::Elif if prev_catcher_kwd == "else" => {
                                return Err(ZyxtError::from_pos(position).error_2_1_7("elif".to_string()))
                            },
                            Keyword::Else if prev_catcher_kwd == "else" => {
                                return Err(ZyxtError::from_pos(position).error_2_1_7("else".to_string()))
                            },
                            _ => break
                        };
                    } else {break}
                    prev_catcher_kwd = catcher_kwd;
                    cursor += 1;
                    catcher_selected = &elements[cursor];
                    let condition= if catcher_kwd == "else" {
                        Element::NullElement
                    } else if let Element::Block {..} = catcher_selected {
                        cursor += 1;
                        catcher_selected.clone()
                    } else {
                        let mut catcher = vec![elements[cursor].clone()];
                        loop {
                            cursor += 1;
                            let catcher_selected = &elements[cursor];
                            if let Element::Block {..} = catcher_selected {break}
                            else {catcher.push(catcher_selected.clone());}
                        };
                        parse_expr(catcher, filename)?
                    };
                    catcher_selected = &elements[cursor];
                    if let Element::Block {content, ..} = catcher_selected {
                        conditions.push(Condition {
                            condition,
                            if_true: content.clone()
                        })
                    } else {
                        return Err(ZyxtError::from_pos(position).error_2_1_8("TODO".to_string()))
                    }
                    cursor += 1;
                    if cursor == elements.len() {break;}
                }
                new_elements.push(Element::If {
                    position: start_pos,
                    conditions
                });
                cursor -= 1;
            },
            Keyword::Elif | Keyword::Else => {
                return Err(ZyxtError::from_pos(position)
                    .error_2_1_9(if kwd == &Keyword::Elif {"elif"} else {"else"}.to_string()))
            },
            _ => new_elements.push(selected.clone())
        }} else {new_elements.push(selected.clone());}
        cursor += 1;
    }
    Ok(new_elements)
}

fn parse_expr(mut elements: Vec<Element>, filename: &String) -> Result<Element, ZyxtError> {
    if elements.len() > 1 {
        elements = parse_parens(elements, filename)?;
    }
    elements = parse_if_expr(elements, filename)?;
    elements = parse_procs_and_fns(elements, filename)?;
    elements = parse_vars_literals_and_calls(elements, filename)?;
    elements = parse_delete_expr(elements, filename)?;
    elements = parse_return_expr(elements, filename)?;
    elements = parse_declaration_expr(elements, filename)?;
    elements = parse_assignment_oprs(elements, filename)?;
    elements = parse_normal_oprs(elements, filename)?;
    elements = parse_un_oprs(elements, filename)?;
    if elements.len() > 1 {
        return Err(ZyxtError::from_pos(&elements[1].get_pos()).error_2_1_0("TODO".to_string()))
    }
    Ok(elements.get(0).unwrap_or(&Element::NullElement).clone())
}

fn parse_block(input: Vec<Element>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    split_between(TokenType::StatementEnd,
                  TokenType::OpenCurlyParen,
                  TokenType::CloseCurlyParen,
                  input, filename, true)
}

pub fn parse_token_list(mut input: Vec<Token>, filename: &String) -> Result<Vec<Element>, ZyxtError> {
    let mut comments: Vec<Element> = vec![];

    // detect & remove comments
    for token in input.iter() {
        if token.type_ == TokenType::Comment {
            comments.push(Element::Comment {
                position: token.position.clone(),
                content: token.value.clone()
            })
        } else if [TokenType::CommentStart,
            TokenType::CommentEnd,
            TokenType::MultilineCommentStart,
            TokenType::MultilineCommentEnd].contains(&token.type_) {
            return Err(ZyxtError::from_pos(&token.position).error_2_1_10(token.value.clone()))
        }
    }
    // remove quotes around LiteralStrings
    for token in input.iter_mut() {
        if token.type_ == TokenType::LiteralString {
            token.value = token.value[1..token.value.len()-1].to_string()
        }
    }

    input = input.into_iter().filter(|token| token.type_ != TokenType::Comment).collect();

    // generate and return an AST for each expression
    parse_block(input.into_iter().map(|t| Element::Token(t))
                    .collect::<Vec<Element>>(), filename)
}