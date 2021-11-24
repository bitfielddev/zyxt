fn parse_expression(pre_elements []ElementGroup, filename string) []ElementGroup {
    mut cursor := 0
    mut selected := ElementGroup(Token{})
    mut elements := pre_elements.clone()
    mut new_elements := []ElementGroup{}
    mut catcher := []ElementGroup{}

    // parse ()s
    for cursor < elements.len {
        selected = elements[cursor]
        if mut selected is Token {
            mut prev_element := ElementGroup(Token{})
            if cursor != 0 {prev_element = elements[cursor-1]}
            if selected.type_ == .open_paren // if selected is Token and is (
            && mut prev_element is Token {
                if cursor == 0 // and the element is the first token
                || (TokenCategory.literal !in prev_element.categories
                && prev_element.type_ !in [.variable, .close_paren, .close_square_paren]) { // or the previous element isn't a literal, variable or )/]
                    mut paren_level := 0 // then it's regular parentheses
                    catch_loop: for {
                        cursor++
                        catcher_selected := elements[cursor]
                        if mut catcher_selected is Token {
                            if catcher_selected.type_ == .close_paren && paren_level == 0 {break catch_loop}
                            else if catcher_selected.type_ == .close_paren {paren_level--}
                            else if catcher_selected.type_ == .open_paren {paren_level++}
                        }
                        catcher << catcher_selected
                    }
                    new_elements << parse_expression(catcher, filename).clone()
                    catcher.clear()
                } else {new_elements << selected} // or else, it's function arguments
            } else {new_elements << selected}
        } else {new_elements << selected}
        
        cursor++
    }
    elements = new_elements.clone()
    new_elements.clear()
    cursor = 0

    // parse literals
    for cursor < elements.len {
        selected = elements[cursor]
        if mut selected is Token {
            if selected.type_ == .literal_misc {
                match selected.value {
                    "true", "false" {new_elements << ElementGroup(Literal{
                        line: selected.line
                        column: selected.column
                        type_: Variable{name: "bool"}
                        content: selected.value
                    })} "null" {new_elements << ElementGroup(Literal{
                        line: selected.line
                        column: selected.column
                        type_: Variable{name: "#any"}
                        content: selected.value
                    })} "inf", "undef" {new_elements << ElementGroup(Literal{
                        line: selected.line
                        column: selected.column
                        type_: Variable{name: "#num"}
                        content: selected.value
                    })} else {panic(selected.value)}
                }
            } else if selected.type_ == .literal_number {
                if selected.value.contains(".") {
                    new_elements << ElementGroup(Literal{
                        line: selected.line
                        column: selected.column
                        type_: Variable{name: "double"}
                        content: selected.value
                    })
                } else {
                    new_elements << ElementGroup(Literal{
                        line: selected.line
                        column: selected.column
                        type_: Variable{name: "int"}
                        content: selected.value
                    })
                }
            } else if selected.type_ == .literal_string {
                new_elements << ElementGroup(Literal{
                    line: selected.line
                    column: selected.column
                    type_: Variable{name: "str"}
                    content: selected.value
                })
            } else {new_elements << selected}
        } else {new_elements << selected}
        cursor++
    }
    elements = new_elements.clone()
    new_elements.clear()
    cursor = 0

    // parse variables and function arguments
    mut catcher2 := ElementGroup(NullElement{})
    catcher.clear()
    for cursor < elements.len {
        selected = elements[cursor]
        if mut selected is Token {
            match selected.type_ {
                 .dot_opr {
                    if cursor == 0 {
                        error_pos(filename, selected.line, selected.column)
                        error_2_1(".") // could be enum but thats for later
                    } else if cursor == elements.len-1 {
                        error_pos(filename, selected.line, selected.column)
                        error_2_1(".") // definitely at the wrong place
                    }
                    mut prev_element := elements[cursor-1]
                    mut next_element := elements[cursor+1]
                    if mut next_element is Token {
                        if next_element.type_ != .variable {
                            error_pos(filename, next_element.line, next_element.column)
                            error_2_1(next_element.value) //definitely at the wrong place
                        }
                    } else if mut next_element is Literal {
                        error_pos(filename, next_element.line, next_element.column)
                        error_2_1(next_element.content) //definitely at the wrong place
                    }
                    if mut prev_element is Token && mut next_element is Token {
                        if prev_element.type_ !in [.close_square_paren, .close_paren, .variable] {
                            error_pos(filename, selected.line, selected.column)
                            error_2_1(".") // could be enum but thats for later
                        }
                        catcher2 = ElementGroup(VariableAttribute{
                            line: next_element.line
                            column: next_element.column
                            name: next_element.value
                            parent: catcher2
                        })
                        cursor++
                    } else if mut next_element is Token {
                        catcher2 = ElementGroup(VariableAttribute{
                            line: next_element.line
                            column: next_element.column
                            name: next_element.value
                            parent: catcher2
                        })
                    } else {
                        error_pos(filename, selected.line, selected.column)
                        error_2_1(".") //definitely at the wrong place
                    }
                 } .variable {
                    if catcher2 != ElementGroup(NullElement{}) {new_elements << catcher2}
                    catcher2 = ElementGroup(Variable{
                        line: selected.line
                        column: selected.column
                        name: selected.value
                    })
                 } .open_paren {
                    if cursor == 0 {
                        error_pos(filename, selected.line, selected.column)
                        error_2_1("(") // parens should have been settled in the first part
                    }
                    mut paren_level := 0
                    mut args := []ElementGroup{}
                    catcher.clear()
                    catch_loop2: for {
                        cursor++
                        catcher_selected := elements[cursor]
                        if mut catcher_selected is Token {
                            if catcher_selected.type_ == .close_paren && paren_level == 0 {break catch_loop2}
                            else if catcher_selected.type_ == .comma && paren_level == 0 {
                                args << parse_expression(catcher, filename).clone()
                                catcher.clear()
                            }
                            else if catcher_selected.type_ == .close_paren {paren_level--}
                            else if catcher_selected.type_ == .open_paren {paren_level++}
                        }
                        catcher << catcher_selected
                    }
                    args << parse_expression(catcher, filename).clone()
                    catcher.clear()
                    catcher2 = ElementGroup(Call{
                        line: selected.line
                        column: selected.column
                        called: catcher2
                        args: args.clone()
                    })
                } else {
                    if catcher2 != ElementGroup(NullElement{}) {new_elements << catcher2}
                    catcher2 = ElementGroup(NullElement{})
                    new_elements << selected
                }
            }
        } else if mut selected is Literal || mut selected is LiteralFunction {
            if catcher2 != ElementGroup(NullElement{}) {new_elements << catcher2}
            catcher2 = selected
        } else {
            if catcher2 != ElementGroup(NullElement{}) {new_elements << catcher2}
            catcher2 = ElementGroup(NullElement{})
            new_elements << selected
        }
        cursor++
    }
    if catcher2 != ElementGroup(NullElement{}) {new_elements << catcher2}
    catcher2 = ElementGroup(NullElement{})
    elements = new_elements.clone()
    new_elements.clear()
    cursor = 0

    for cursor < elements.len {
        selected = elements[cursor]
        if mut selected is Token {

        }
    }


    println(elements)

    return elements
}

fn parse(preinput []Token, filename string) []string {
    mut input := preinput.clone()
    mut comments := []Comment{}

    // detect and remove comments
    for token in input {
        if token.type_ == .comment {
            comments << Comment{
                line: token.line
                column: token.column
                content: token.value
            }
        }
    }
    input = input.filter(fn (token Token) bool {
        return token.type_ !in [.comment_start, .comment_end, .multiline_comment_start, .multiline_comment_end, .comment]
    })

    // separate token inputs into statements
    mut token_statements := [][]ElementGroup{}
    mut token_stack := []ElementGroup{}
    for token in input {
        if token.type_ == .statement_end {
            token_statements << token_stack.clone()
            token_stack.clear()
        } else {token_stack << ElementGroup(token)}
    }

    // generate an AST for each statement
    for statement in token_statements {
        parse_expression(statement, filename)
    }

    return []
}