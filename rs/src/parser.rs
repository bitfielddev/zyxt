use crate::syntax::lexing::{TokenCategory, TokenType};
use crate::syntax::parsing::Element;
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