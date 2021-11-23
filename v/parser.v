struct Element {
    line int
    column int
}
struct Comment {
    Element
    content string
}

struct Literal {
    Element
    type_ string
    content any
}
struct LiteralCall {
    Element
    Literal
    args []ElementGroup
    //kwargs map[Literal]ElementGroup
}

struct Variable {
    Element
    name string
}
struct VariableFunction {
    Element
    Variable
    args []ElementGroup
    //kwargs map[Literal]ElementGroup
}
struct VariableAttribute {
    Element
    Variable
    parent ElementGroup
}
struct VariableAttributeFunction {
    Element
    Variable
    VariableAttribute
    VariableFunction
}

struct Statement {
    content []Token
}

type ElementGroup = Element
                  | Comment
                  | Literal | LiteralCall
                  | Variable | VariableFunction | VariableAttribute | VariableAttributeFunction
                  | Token

const returning_element_list = ["Variable",
                                "VariableFunction",
                                "VariableAttribute",
                                "VariableAttributeFunction",
                                "Literal"]

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
                if TokenCategory.literal !in prev_element.categories  // and the prev element is the fist token
                && prev_element.type_ !in [.variable, .close_paren, .close_square_paren] {// or it isn't a literal, variable or )/]
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

    // parse variables and function arguments
    mut catcher2 := ElementGroup{}
    catcher.clear()
    for cursor < elements.len {
        selected = elements[cursor]
        if mut selected is Token {
            if selected.type_ == .dot_opr {
                if cursor == 0 {
                    error_pos(filename, selected.line, selected.column)
                    error_2_1(".") // could be enum but thats for later
                } else if cursor == elements.len-1 {
                    error_pos(filename, selected.line, selected.column)
                    error_2_1(".") // definitely at the wrong place
                }
                mut prev_element := elements[cursor-1]
                mut next_element := elements[cursor+1]
                if mut prev_element is Token && mut next_element is Token {
                    if prev_element.type_ !in [.close_square_paren, .close_paren, .variable] {
                        error_pos(filename, selected.line, selected.column)
                        error_2_1(".") // could be enum but thats for later
                    } else if next_element.type_ != .variable {
                        error_pos(filename, next_element.line, next_element.column)
                        error_2_1(next_element.value) //definitely at the wrong place
                    }
                    catcher2 = ElementGroup(VariableAttribute{
                        line: next_element.line
                        column: next_element.column
                        name: next_element.value
                        parent: catcher2
                    })
                    cursor++
                }
            } else if selected.type_ == .variable {
                new_elements << catcher2
                catcher2 = ElementGroup(Variable{
                    line: selected.line
                    column: selected.column
                    name: selected.value
                })
            } else if selected.type_ == .open_paren {
                if cursor == 0 {
                    error_pos(filename, selected.line, selected.column)
                    error_2_1("(") // parens should have been settled in the first part
                }
                mut paren_level := 0
                mut args := []ElementGroup{}
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
                if mut catcher2 is Variable {
                    catcher2 = ElementGroup(VariableFunction{
                        line: catcher2.line
                        column: catcher2.column
                        name: catcher2.name
                        args: args.clone()
                    })
                } else if mut catcher2 is VariableAttribute {
                    catcher2 = ElementGroup(VariableAttributeFunction{
                        line: catcher2.line
                        column: catcher2.column
                        name: catcher2.name
                        parent: catcher2.parent
                        args: args.clone()
                    })
                } else if mut catcher2 is Literal {
                    catcher2 = ElementGroup(LiteralCall{
                        line: catcher2.line
                        column: catcher2.column
                        type_: catcher2.type_
                        content: catcher2.content
                        args: args.clone()
                    })
                } else if mut catcher2 is LiteralCall {
                    catcher2 = ElementGroup(LiteralCall{
                        line: catcher2.line
                        column: catcher2.column
                        type_: catcher2.type_
                        content: catcher2.content
                        args: args.clone()
                    })
                } else if mut catcher2 is VariableFunction {
                    catcher2 = ElementGroup(LiteralCall{
                        line: catcher2.line
                        column: catcher2.column
                        type_: "#A"
                        content: catcher2
                        args: args.clone()
                    })
                } else if mut catcher2 is VariableAttributeFunction {
                    catcher2 = ElementGroup(LiteralCall{
                        line: catcher2.line
                        column: catcher2.column
                        type_: "#A"
                        content: catcher2
                        args: args.clone()
                    })
                }
            } else {
                new_elements << catcher2
                catcher2 = ElementGroup{}
                new_elements << selected
            }
        } else if mut selected is Literal {
            new_elements << catcher2
            catcher2 = ElementGroup(Literal{
                line: selected.line
                column: selected.column
                content: selected.content
                type_: selected.type_
            })
        } else {
            new_elements << catcher2
            catcher2 = ElementGroup{}
            new_elements << selected
        }
        cursor++
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