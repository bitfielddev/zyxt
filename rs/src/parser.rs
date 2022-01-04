use crate::syntax::{Element, TokenCategory};
use crate::syntax::TokenType;
use crate::Token;

fn parse_expression(elements: Vec<Element>, filename: &String) -> Vec<&Element> {
    let mut cursor = 0;
    let mut selected = &Element::NullElement;
    let mut new_elements: Vec<&Element> = vec![];
    let mut catcher = vec![];

    // parse ()s
    while cursor < elements.len() {
        selected = &elements[cursor];
        if let &Element::Token(selected) = selected {
        if selected.type_ == TokenType::OpenParen {
            let mut prev_element = &Element::Token(Token);
            if cursor != 0 { prev_element = &elements[cursor - 1]; }
            if let &Element::Token(prev_element) = prev_element {// if selected is Token and is (
                if cursor == 0
                || (!prev_element.categories.contains(&TokenCategory::Literal)
                && ![TokenType::Variable,
                    TokenType::CloseParen,
                    TokenType::CloseSquareParen].contains(&prev_element.type_)) {
                    let mut paren_level = 0;
                    'catch_loop: loop {
                        cursor += 1;
                        let catcher_selected = &elements[cursor];
                        if let &Element::Token(catcher_selected) = catcher_selected {
                            if catcher_selected.type_ == TokenType::CloseParen && paren_level == 0 {break 'catch_loop;}
                            else if catcher_selected.type_ == TokenType::CloseParen {paren_level -= 1;}
                            else if catcher_selected.type_ == TokenType::OpenParen {paren_level += 1;}
                        }
                        catcher.push(catcher_selected)
                    }
                    new_elements.append(&mut parse_expression(catcher.into_iter().cloned().collect(), &filename)).clone();
                    catcher.clear()
                } else {new_elements.push(&Element::Token(selected))} // or else it's function args
            } else {new_elements.push(&Element::Token(selected))}
        } else {new_elements.push(&Element::Token(selected))}
        } else {new_elements.push(selected)}
    }

    vec![]
}