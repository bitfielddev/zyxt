use crate::syntax::lexing::{TokenCategory, TokenType, UnarySide};
use crate::syntax::parsing::{Element, get_order};
use crate::{errors, Token};
use crate::lexer::Position;



fn parse_parens(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    // parse ()s
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            if selected.type_ == TokenType::OpenParen {
                let mut prev_element = &Element::Token(Token{..Default::default()});
                if cursor != 0 { prev_element = &elements[cursor - 1]; }
                if let Element::Token(prev_element) = prev_element {// if selected is Token and is (
                    if cursor == 0
                        || (!prev_element.categories.contains(&TokenCategory::Literal)
                        && ![TokenType::Variable,
                        TokenType::CloseParen,
                        TokenType::CloseSquareParen].contains(&prev_element.type_)) {
                        let mut paren_level = 0;
                        let mut catcher: Vec<Element> = vec![];
                        'catch_loop: loop {
                            cursor += 1;
                            let catcher_selected = &elements[cursor];
                            if let Element::Token(catcher_selected) = catcher_selected {
                                if catcher_selected.type_ == TokenType::CloseParen && paren_level == 0 {break 'catch_loop;}
                                else if catcher_selected.type_ == TokenType::CloseParen {paren_level -= 1;}
                                else if catcher_selected.type_ == TokenType::OpenParen {paren_level += 1;}
                            }
                            catcher.push(catcher_selected.clone())
                        }
                        new_elements.append(&mut parse_expression(catcher.clone(), &filename));
                    } else {new_elements.push(Element::Token(selected.clone()))} // or else it's function args
                } else {new_elements.push(Element::Token(selected.clone()))}
            } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }

    new_elements
}

fn parse_literals(elements: Vec<Element>) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            if selected.categories.contains(&TokenCategory::Literal) {
                new_elements.push(Element::Literal {
                    position: selected.position.clone(),
                    type_: Box::from(Element::Variable {
                        position: selected.position.clone(),
                        name: if selected.type_ == TokenType::LiteralMisc {
                            match &*selected.value {
                                "true" | "false" => "bool",
                                "null" => "#any",
                                "inf" | "undef" => "#num",
                                _ => panic!("{}", selected.value)
                            }
                        } else if selected.type_ == TokenType::LiteralNumber{
                            if selected.value.contains(".") {"double"} else {"int"}
                        } else {"str"}.to_string(),
                        parent: Box::new(Element::NullElement)
                    }),
                    content: selected.value.clone()
                });
            } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    new_elements
}

fn parse_attrs_and_calls(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut catcher2: Element = Element::NullElement;

    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected { match selected.type_ {
            TokenType::DotOpr => {
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
                    if ![TokenType::CloseSquareParen, TokenType::CloseParen, TokenType::Variable].contains(&prev_element.type_) {
                        errors::error_pos(&selected.position);
                        errors::error_2_1(String::from(".")); //could be enum but thats for later
                    }
                    catcher2 = Element::Variable{
                        position: next_element.position.clone(),
                        name: next_element.value.clone(),
                        parent: Box::new(catcher2)
                    };
                    cursor += 1;
                } else if let Element::Token(next_element) = next_element {
                    catcher2 = Element::Variable{
                        position: next_element.position.clone(),
                        name: next_element.value.clone(),
                        parent: Box::new(catcher2)
                    };
                } else {
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from(".")); // definitely at the wrong place
                }

            }
            TokenType::Variable => {
                if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
                catcher2 = Element::Variable {
                    position: selected.position.clone(),
                    name: selected.value.clone(),
                    parent: Box::new(Element::NullElement)
                }
            }
            TokenType::OpenParen => {
                if cursor == 0 {
                    errors::error_pos(&selected.position);
                    errors::error_2_1(String::from("(")); // parens should have been settled in the first part
                }
                let mut paren_level = 0;
                let mut args: Vec<Element> = vec![];
                catcher.clear();
                'catch_loop2: loop {
                    cursor += 1;
                    let catcher_selected = &elements[cursor];
                    if let Element::Token(catcher_selected) = catcher_selected {
                        if catcher_selected.type_ == TokenType::CloseParen && paren_level == 0 {break 'catch_loop2;}
                        else if catcher_selected.type_ == TokenType::Comma && paren_level == 0 {
                            let result = parse_expression(catcher.clone(), filename);
                            if result.len() == 0 {
                                errors::error_pos(&Position{filename: filename.clone(), line: 0, column: 0});
                                errors::error_2_3("???".to_string(), args.len()); // TODO
                            }
                            args.push(result[0].clone());
                            catcher.clear();
                        }
                        else if catcher_selected.type_ == TokenType::CloseParen {paren_level -= 1;}
                        else if catcher_selected.type_ == TokenType::OpenParen {paren_level += 1;}
                    }
                    catcher.push(catcher_selected.clone());
                }
                if catcher.len() != 0 {
                    let result = parse_expression(catcher.clone(), filename);
                    if result.len() == 0 {
                        errors::error_pos(&Position{filename: filename.clone(), line: 0, column: 0});
                        errors::error_2_3("???".to_string(), args.len()); // TODO
                    }
                    args.push(result[0].clone());
                }
                catcher.clear();
                catcher2 = Element::Call {
                    position: selected.position.clone(),
                    called: Box::new(catcher2),
                    args
                }
            }
            _ => {
                if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
                catcher2 = Element::NullElement;
                new_elements.push(Element::Token(selected.clone()));
            }
        }} else if let Element::Literal {..} = selected {
            if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
            catcher2 = selected.clone();
        } else if let Element::Call {called, ..} = selected {
            if let Element::Literal {..} = **called {
                if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
                catcher2 = selected.clone();
            }} else {
            if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
            catcher2 = Element::NullElement;
            new_elements.push(selected.clone());
        }
        cursor += 1;
    }
    if catcher2 != Element::NullElement {new_elements.push(catcher2);}
    //catcher2 = Element::NullElement;
    new_elements
}

fn parse_operators(elements: Vec<Element>, filename: &String) -> Vec<Element> {
    if elements.len() == 0 {return vec![]}
    let mut highest_order_index: usize = 0;
    let mut highest_order = 0;
    for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, value, .. }) = ele {
            if i == 0 || i == elements.len()-1 {
                errors::error_pos(position);
                errors::error_2_1(value.clone());
            }
            if get_order(&opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(&opr_type);
            }}
    }
    if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), position, ..}) = &elements[highest_order_index] {
        return vec![Element::BinaryOpr {
            position: position.clone(),
            type_: *opr_type,
            operand1: Box::new(parse_expression(elements[..highest_order_index].to_vec(), filename)[0].clone()),
            operand2: Box::new(parse_expression(elements[highest_order_index+1..].to_vec(), filename)[0].clone())
        }]
    } else {elements}
    // TODO unary operators
}

fn parse_expression(mut elements: Vec<Element>, filename: &String) -> Vec<Element> {
    elements = parse_parens(elements, filename);
    elements = parse_operators(elements, filename);
    elements = parse_literals(elements);
    elements = parse_attrs_and_calls(elements, filename);

    // TODO assignment operators above

    elements
}

fn parse_statement(mut elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut statement_detected = false;

    // parse declaration statement
    let mut flag_pos = None;
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::Flag(_), ..}) = selected {flag_pos = Some(cursor);}
        if let Element::Token(Token{type_: TokenType::DeclarationStmt, position, ..}) = selected {
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
            new_elements.push(Element::DeclarationStmt {
                position: position.clone(),
                variable: Box::new(declared_var.clone()),
                content: Box::new(parse_expression(elements[cursor+1..].to_vec(), filename).get(0).unwrap().clone()),
                flags,
                type_: Box::new(Element::NullElement) // TODO type later
            });
            statement_detected = true;
            break;
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    elements = new_elements.clone();
    new_elements.clear();

    if !statement_detected {elements = parse_expression(elements, filename)}

    for ele in elements.iter() {println!("{}", ele)}
    //cursor = 0;
    elements
}

pub fn parse_statements(mut input: Vec<Token>, filename: &String) -> Vec<Element> {
    let mut comments: Vec<Element> = vec![];

    // detect & remove comments
    for token in input.iter() { if token.type_ == TokenType::Comment {
        comments.push(Element::Comment {
            position: token.position.clone(),
            content: token.value.clone()
        })
    }}

    input = input.into_iter().filter(|token| ![
        TokenType::CommentStart,
        TokenType::CommentEnd,
        TokenType::MultilineCommentStart,
        TokenType::MultilineCommentEnd,
        TokenType::Comment].contains(&token.type_)).collect();
    // separate token inputs into statements
    let mut token_statements: Vec<Vec<Element>> = vec![];
    let mut token_stack: Vec<Element> = vec![];
    for token in input { if token.type_ == TokenType::StatementEnd {
        token_statements.push(token_stack.clone());
        token_stack.clear()
    } else {token_stack.push(Element::Token(token));}}
    // generate an AST for each statement
    for statement in token_statements {parse_statement(statement, filename);}

    vec![]
}