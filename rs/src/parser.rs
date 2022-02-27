use std::collections::HashMap;
use crate::syntax::token::{TokenCategory, TokenType, get_order, Side, OprType, Keyword};
use crate::syntax::element::{Condition, Element};
use crate::{errors, Token};
use crate::lexer::Position;

fn catch_between(opening: TokenType, closing: TokenType,
                 elements: &Vec<Element>, cursor: &mut usize) -> Vec<Element> {
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
            if opening == TokenType::Null {
                errors::error_pos(&paren_pos);
                errors::error_2_1("TODO".to_string())
            } else {
                errors::error_pos(&paren_pos);
                errors::error_2_0_1(opening_char)
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
    catcher
}

fn split_between(divider: TokenType, opening: TokenType, closing: TokenType,
                 elements: Vec<Element>, filename: &String, ignore_empty: bool) -> Vec<Element> {
    let mut out: Vec<Element> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut paren_level = 0;
    for element in elements {
        if let Element::Token(Token{type_, ..}) = element {
            if type_ == divider && paren_level == 0 {
                if !ignore_empty && catcher.len() == 0 {
                    todo!()
                } else if catcher.len() != 0 {
                    out.push(parse_expr(catcher.clone(), filename));
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
        errors::error_pos(&Default::default()); // TODO
        errors::error_2_0_1(match opening {
            TokenType::OpenParen => '(',
            TokenType::OpenSquareParen => '[',
            TokenType::OpenCurlyParen => '{',
            TokenType::OpenAngleBracket => '<',
            _ => '?'
        })
    }
    if !ignore_empty && catcher.len() == 0 {
        todo!()
    } else if catcher.len() != 0 {
        out.push(parse_expr(catcher.clone(), filename));
    }
    out
}

fn parse_parens(elements: Vec<Element>, filename: &String) -> Vec<Element> {
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
                            &elements, &mut cursor);
                        new_elements.push(parse_expr(paren_contents, &filename));
                    } else {new_elements.push(Element::Token(selected.clone()))} // or else it's function args
                } else {new_elements.push(Element::Token(selected.clone()))}
            } else if selected.type_ == TokenType::OpenCurlyParen { // blocks, {
                let paren_contents = catch_between(TokenType::OpenCurlyParen,
                                                   TokenType::CloseCurlyParen,
                                                   &elements, &mut cursor);
                new_elements.push(Element::Block {
                    position: selected.position.clone(),
                    content: parse_block(paren_contents, filename)
                });
            } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }

    new_elements
}

fn parse_vars_literals_and_calls(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut catcher: Element = Element::NullElement;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected { match selected.type_ {
            TokenType::DotOpr => { // TODO rewrite this
                if cursor == 0 {
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from(".")); // could be enum but thats for later
                } else if cursor == elements.len()-1 {
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from(".")); // definitely at the wrong place
                }
                let prev_element = &elements[cursor-1];
                let next_element = &elements[cursor+1];
                if let Element::Token(next_element) = next_element {
                    if next_element.type_ != TokenType::Variable {
                        errors::error_pos(&next_element.position);
                        errors::error_2_1(next_element.value.clone());
                    }} else if let Element::Literal{position, content, ..} = next_element {
                    errors::error_pos(position);
                    errors::error_2_1(content.clone())
                }
                if let (Element::Token(prev_element), Element::Token(next_element)) = (prev_element, next_element) {
                    if !prev_element.categories.contains(&TokenCategory::ValueEnd) {
                        errors::error_pos(&selected.position);
                        errors::error_2_1(String::from(".")); //could be enum but thats for later
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
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from(".")); // definitely at the wrong place
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
                    type_: Box::from(Element::Variable {
                        position: selected.position.clone(),
                        name: if selected.type_ == TokenType::LiteralMisc {
                            match &*selected.value {
                                "true" | "false" => "bool",
                                "null" => "#null",
                                "inf" | "undef" => "#num",
                                _ => panic!("{}", selected.value)
                            }
                        } else if selected.type_ == TokenType::LiteralNumber{
                            if selected.value.contains(".") {"f64"} else {"i32"}
                        } else {"str"}.to_string(),
                        parent: Box::new(Element::NullElement)
                    }),
                    content: selected.value.clone()
                }
            }
            TokenType::CloseParen => {
                errors::error_pos(&selected.position);
                errors::error_2_0_2(')')
            }
            TokenType::OpenParen => {
                if cursor == 0 {
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from("(")); // parens should have been settled in the first part
                }
                let args: Vec<Element> = split_between(TokenType::Comma,
                    TokenType::OpenParen, TokenType::CloseParen,
                    catch_between(TokenType::OpenParen,
                                  TokenType::CloseParen,
                                  &elements, &mut cursor),
                    filename, false);
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
            catcher = Element::NullElement;
            new_elements.push(selected.clone());
        }
        cursor += 1;
    }
    if catcher != Element::NullElement {new_elements.push(catcher);}
    //catcher2 = Element::NullElement;
    new_elements
}

fn parse_assignment_oprs(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    if elements.len() == 0 {return vec![]}
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::AssignmentOpr(opr_type), position, ..}) = ele {
            if i == 0 || i == elements.len()-1 {
                errors::error_pos(position);
                errors::error_2_1("TODO".to_string());
            }
            let variable = parse_expr(vec![elements[i-1].clone()], filename);
            let content = if opr_type == &OprType::Null {
                parse_expr(elements[i+1..].to_vec(), filename)
            } else {
                Element::BinaryOpr {
                    position: position.clone(),
                    type_: opr_type.clone(),
                    operand1: Box::new(variable.clone()),
                    operand2: Box::new(parse_expr(elements[i+1..].to_vec(), filename))
                }
            };

            return elements[..i-1].to_vec().into_iter()
                .chain(vec![Element::Set {
                    position: position.clone(),
                    variable: Box::new(variable),
                    content: Box::new(content)
                }]).collect::<Vec<Element>>()
        }
    }
    elements
}

fn parse_un_oprs(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    if elements.len() == 0 {return vec![]}
    for (i, ele) in elements.iter().enumerate().rev() {
        if let Element::Token(Token{type_: TokenType::UnaryOpr(opr_type, opr_side), position, ..}) = ele {
            if opr_side == &Side::Left {
                if i == elements.len()-1 {
                    errors::error_pos(position);
                    errors::error_2_1("TODO".to_string())
                }
                return elements[..i].to_vec().into_iter()
                    .chain(vec![Element::UnaryOpr {
                        position: position.clone(),
                        type_: *opr_type,
                        operand: Box::new(parse_expr(elements[i+1..].to_vec(), filename))
                    }]).collect::<Vec<Element>>()
            } else if opr_side == &Side::Right {
                if i == 0 {
                    errors::error_pos(position);
                    errors::error_2_1("TODO".to_string())
                }
                return vec![Element::UnaryOpr {
                    position: position.clone(),
                    type_: *opr_type,
                    operand: Box::new(parse_expr(elements[..i].to_vec(), filename))
                }].into_iter()
                    .chain(elements[i+1..].to_vec())
                    .collect::<Vec<Element>>();
            }
        }
    }
    elements
}

fn parse_normal_oprs(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    if elements.len() == 0 {return vec![]}
    let mut highest_order_index: usize = 0;
    let mut highest_order = 0;
    let mut opr_detected = false;
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, value, .. }) = ele {
            if i == 0 || i == elements.len()-1 {
                errors::error_pos(position);
                errors::error_2_1(value.clone());
            }
            if get_order(&opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(&opr_type);
                opr_detected = true
            }}
    }
    if !opr_detected {elements}
    else if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, ..}) = &elements[highest_order_index] {
        vec![Element::BinaryOpr {
            position: position.clone(),
            type_: *opr_type,
            operand1: Box::new(parse_expr(elements[..highest_order_index].to_vec(), filename)),
            operand2: Box::new(parse_expr(elements[highest_order_index+1..].to_vec(), filename))
        }]
    } else {elements}
}

fn parse_declaration_expr(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    let mut flag_pos = None;
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::Flag(_), ..}) = selected {flag_pos = Some(cursor);}
        if let Element::Token(Token{type_: TokenType::DeclarationOpr, position, ..}) = selected {
            if cursor == elements.len() - 1 || cursor == 0 {
                errors::error_pos(position);
                errors::error_2_1(String::from(":="));
            }
            let declared_var = &elements[cursor-1];
            let flags = if flag_pos == None {vec![]} else {
                let mut f = vec![];
                for i in flag_pos.unwrap()..cursor-1 {
                    if let Element::Token(Token{type_: TokenType::Flag(flag), ..}) = &elements[i] {
                        f.push(*flag);
                    } else {
                        errors::error_pos(&Position{filename: filename.clone(), line: 0, column: 0});
                        errors::error_2_1(String::from("")); // TODO
                    }
                }
                f
            };
            for _ in 0..flags.len()+1 {new_elements.pop();}
            new_elements.push(Element::Declare {
                position: position.clone(),
                variable: Box::new(parse_expr(vec![declared_var.clone()], filename)),
                content: Box::new(parse_expr(elements[cursor+1..].to_vec(), filename)),
                flags,
                type_: Box::new(Element::NullElement) // TODO type later
            });
            break;
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    new_elements
}

pub fn parse_if_expr(elements: Vec<Element>, filename: &String) -> Vec<Element> {
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
                            Keyword::Elif => "elif",
                            Keyword::Else if prev_catcher_kwd != "else" => "else",
                            _ => {
                                errors::error_pos(position);
                                errors::error_2_1("TODO".to_string())
                            },
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
                        parse_expr(catcher, filename)
                    };
                    catcher_selected = &elements[cursor];
                    if let Element::Block {content, ..} = catcher_selected {
                        conditions.push(Condition {
                            condition,
                            if_true: content.clone()
                        })
                    } else {
                        errors::error_pos(position);
                        errors::error_2_1("TODO".to_string())
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
                errors::error_pos(position);
                errors::error_2_1(if kwd == &Keyword::Elif {"elif"} else {"else"}.to_string())
            },
            _ => new_elements.push(selected.clone())
        }} else {new_elements.push(selected.clone());}
        cursor += 1;
    }
    new_elements
}

fn parse_expr(mut elements: Vec<Element>, filename: &String) -> Element {
    if elements.len() > 1 {
        elements = parse_parens(elements, filename);
        elements = parse_if_expr(elements, filename);
    }
    elements = parse_vars_literals_and_calls(elements, filename);
    if elements.len() > 1 {
        elements = parse_declaration_expr(elements, filename);
        elements = parse_assignment_oprs(elements, filename);
        elements = parse_normal_oprs(elements, filename);
        elements = parse_un_oprs(elements, filename);
    }
    if elements.len() > 1 {
        errors::error_pos(&elements[1].get_pos());
        errors::error_2_1("TODO".to_string());
    }
    elements.get(0).unwrap_or(&Element::NullElement).clone()
}

fn parse_block(input: Vec<Element>, filename: &String) -> Vec<Element> {
    split_between(TokenType::StatementEnd,
                  TokenType::OpenCurlyParen,
                  TokenType::CloseCurlyParen,
                  input, filename, true)
}

pub fn parse_token_list(mut input: Vec<Token>, filename: &String) -> Vec<Element> {
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
            errors::error_pos(&token.position);
            errors::error_2_1(token.value.clone());
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