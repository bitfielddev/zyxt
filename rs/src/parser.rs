use crate::syntax::lexing::{TokenCategory, TokenType, UnarySide};
use crate::syntax::parsing::{Element, get_order, OprType};
use crate::{errors, Token};

fn parse_expression(mut elements: Vec<Element>, filename: &String) -> Vec<Element> {
    let mut cursor = 0;
    let mut selected;
    let mut new_elements: Vec<Element> = vec![];
    let mut catcher: Vec<Element> = vec![];
    let mut catcher2: Element = Element::NullElement;

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
                    catcher.clear()
                } else {new_elements.push(Element::Token(selected.clone()))} // or else it's function args
            } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(Element::Token(selected.clone()))}
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    // parse literals
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected {
            if selected.categories.contains(&TokenCategory::Literal) {
                new_elements.push(Element::Literal {
                    line: selected.line,
                    column: selected.column,
                    type_: Box::from(Element::Variable {
                        line: 0,
                        column: 0,
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
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    // parse variables and function arguments
    catcher.clear();
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(selected) = selected { match selected.type_ {
            TokenType::DotOpr => {
                if cursor == 0 {
                    errors::error_pos(filename, selected.line, selected.column);
                    errors::error_2_1(String::from(".")); // could be enum but thats for later
                } else if cursor == elements.len()-1 {
                    errors::error_pos(filename, selected.line, selected.column);
                    errors::error_2_1(String::from(".")); // definitely at the wrong place
                }
                let prev_element = &elements[cursor-1];
                let next_element = &elements[cursor+1];
                if let Element::Token(next_element) = next_element {
                if next_element.type_ != TokenType::Variable {
                    errors::error_pos(filename, next_element.line, next_element.column);
                    errors::error_2_1(next_element.value.clone());
                }} else if let Element::Literal{line, column, content, ..} = next_element {
                    errors::error_pos(filename, *line, *column);
                    errors::error_2_1(content.clone())
                }
                if let (Element::Token(prev_element), Element::Token(next_element)) = (prev_element, next_element) {
                    if ![TokenType::CloseSquareParen, TokenType::CloseParen, TokenType::Variable].contains(&prev_element.type_) {
                        errors::error_pos(filename, selected.line, selected.column);
                        errors::error_2_1(String::from(".")); //could be enum but thats for later
                    }
                    catcher2 = Element::Variable{
                        line: next_element.line,
                        column: next_element.column,
                        name: next_element.value.clone(),
                        parent: Box::new(catcher2)
                    };
                    cursor += 1;
                } else if let Element::Token(next_element) = next_element {
                    catcher2 = Element::Variable{
                        line: next_element.line,
                        column: next_element.column,
                        name: next_element.value.clone(),
                        parent: Box::new(catcher2)
                    };
                } else {
                    errors::error_pos(filename, selected.line, selected.column);
                    errors::error_2_1(String::from(".")); // definitely at the wrong place
                }

            }
            TokenType::Variable => {
                if catcher2 != Element::NullElement {new_elements.push(catcher2.clone());}
                catcher2 = Element::Variable {
                    line: selected.line,
                    column: selected.column,
                    name: selected.value.clone(),
                    parent: Box::new(Element::NullElement)
                }
            }
            TokenType::OpenParen => {
                if cursor == 0 {
                    errors::error_pos(filename, selected.line, selected.column);
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
                                errors::error_pos(filename, 0, 0);
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
                        errors::error_pos(filename, 0, 0);
                        errors::error_2_3("???".to_string(), args.len()); // TODO
                    }
                    args.push(result[0].clone());
                }
                catcher.clear();
                catcher2 = Element::Call {
                    line: selected.line,
                    column: selected.column,
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
    catcher2 = Element::NullElement;
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    // parse unary operators
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::UnaryOpr(_, side), value, line, column, ..}) = selected {
            if (*side == UnarySide::Left && cursor == elements.len() - 1)
            || (*side == UnarySide::Right && cursor == 0){
                errors::error_pos(filename, *line, *column);
                errors::error_2_1(value.clone());
            }
            let mut unary_opr_queue = vec![selected.clone()];
            let mut catcher_unary;
            let operand;
            if *side == UnarySide::Left { 'catch_loop3: loop {
                cursor += 1;
                if cursor == elements.len() {
                    errors::error_pos(filename, 0, 0); // TODO
                    errors::error_2_1(String::from("")); //TODO
                }
                catcher_unary = &elements[cursor];
                if let Element::Token(Token{type_: TokenType::UnaryOpr(_, UnarySide::Left), .. }) = catcher_unary {
                    unary_opr_queue.push(catcher_unary.clone());
                } else if let Element::Literal{..} = catcher_unary {
                    operand = catcher_unary.clone();
                    break 'catch_loop3;
                } else {
                    errors::error_pos(filename, 0, 0); // TODO
                    errors::error_2_1(String::from("")); //TODO
                }
            } unary_opr_queue = unary_opr_queue.into_iter().rev().collect();
            } else {
                operand = new_elements.last().unwrap().clone();
            }

            let mut new = operand;
            for ele in unary_opr_queue.into_iter() {
            if let Element::Token(Token{line, column, type_: TokenType::UnaryOpr(opr_type, _), ..}) = ele {
                new = Element::UnaryOpr {
                    line, column,
                    type_: opr_type,
                    operand: Box::new(new)
                }
            }}
            if *side == UnarySide::Right {new_elements.pop();}
            new_elements.push(new);
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    // parse binary operators
    fn binary(elements: Vec<Element>) -> Element {
        if elements.len() == 1 {return elements[0].clone();}
        let mut highest_order_index: usize = 0;
        let mut highest_order = 0;
        for (i, ele) in elements.iter().enumerate() {
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), .. }) = ele {
            if i >= highest_order {
                highest_order_index = i;
                highest_order = get_order(&opr_type) as usize;
            }}
        }
        if let Element::Token(Token{type_: TokenType::NormalOpr(opr_type), line, column, ..}) = &elements[highest_order_index] {
            return Element::BinaryOpr {
                line: *line, column: *column,
                type_: *opr_type,
                operand1: Box::new(binary(elements[..highest_order_index].to_vec())),
                operand2: Box::new(binary(elements[highest_order_index+1..].to_vec()))
            }
        } else {panic!("{}", elements[highest_order_index])}
    }
    catcher.clear();
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::NormalOpr(_), line, column, value, .. }) = selected {
            if cursor == elements.len() - 1 || cursor == 0 {
                errors::error_pos(filename, *line, *column);
                errors::error_2_1(value.clone());
            }
            if let Element::Literal {..} | Element::Variable {..} | Element::Call{..} | Element::UnaryOpr{..} = &elements[cursor-1] {} else {
                errors::error_pos(filename, *line, *column);
                errors::error_2_1(value.clone());
            }
            catcher = vec![elements[cursor-1].clone(), selected.clone()];
            'catch_loop4: loop {
                cursor += 1;
                if cursor == elements.len() {break 'catch_loop4;}
                let catcher_selected = &elements[cursor];
                if let Element::Literal {line, column, ..}
                     | Element::Variable {line, column, ..}
                     | Element::Call{line, column, ..}
                     | Element::UnaryOpr{line, column, ..} = catcher_selected {
                if let Element::Literal {..} | Element::Variable {..} | Element::Call{..} | Element::UnaryOpr{..} = catcher.last().unwrap() {
                    errors::error_pos(filename, *line, *column);
                    errors::error_2_1(String::from("")); // TODO
                }}
                if let Element::Token(Token{type_: TokenType::NormalOpr(_), .. }) = catcher_selected {
                if let Element::Token(Token{type_: TokenType::NormalOpr(_), line, column, value, .. }) = catcher.last().unwrap() {
                    errors::error_pos(filename, *line, *column);
                    errors::error_2_1(value.clone());
                }}
                if let Element::Literal {..} | Element::Variable {..} | Element::Call{..} | Element::UnaryOpr{..}
                | Element::Token(Token{type_: TokenType::NormalOpr(_), .. }) = catcher_selected {
                    catcher.push(catcher_selected.clone());
                } else {break 'catch_loop4;}
            }
            new_elements.pop();
            new_elements.push(binary(catcher));
        } else {new_elements.push(selected.clone());}
        cursor += 1;
    }
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    // TODO assignment operators above

    // parse declaration statement
    let mut flag_pos = None;
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let Element::Token(Token{type_: TokenType::Flag(_), ..}) = selected {flag_pos = Some(cursor);}
        if let Element::Token(Token{type_: TokenType::DeclarationStmt, line, column, ..}) = selected {
            if cursor == elements.len() - 1 || cursor == 0 {
                errors::error_pos(filename, *line, *column);
                errors::error_2_1(String::from(":="));
            }
            let declared_var = &elements[cursor-1];
            let flags = if flag_pos == None {vec![]} else {
                let mut f = vec![];
                for i in flag_pos.unwrap()..cursor-1 {
                    if let Element::Token(Token{type_: TokenType::Flag(flag), ..}) = &elements[i] {
                        f.push(*flag);
                    } else {
                        errors::error_pos(filename, 0, 0);
                        errors::error_2_1(String::from("")); // TODO
                    }
                }
                f
            };
            for _ in 0..flags.len()+1 {new_elements.pop();}
            new_elements.push(Element::DeclarationStmt {
                line: *line,
                column: *column,
                variable: Box::new(declared_var.clone()),
                content: Box::new(elements[cursor+1].clone()),
                flags,
                type_: Box::new(Element::NullElement) // TODO type later
            });
            cursor += 1;
        } else {new_elements.push(selected.clone())}
        cursor += 1;
    }
    elements = new_elements.clone();
    new_elements.clear();
    cursor = 0;

    for ele in elements.iter() {println!("{}", ele)}
    elements
}

pub(crate) fn parse(mut input: Vec<Token>, filename: &String) -> Vec<Element> {
    let mut comments: Vec<Element> = vec![];

    // detect & remove comments
    for token in input.iter() { if token.type_ == TokenType::Comment {
        comments.push(Element::Comment {
            line: token.line,
            column: token.column,
            content: token.value.clone()
        })
    }}

    input = input.into_iter().filter(move |token| ![
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
    for statement in token_statements {parse_expression(statement, filename);}

    vec![]
}