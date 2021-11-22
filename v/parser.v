struct Element {
    line int
    column int
}
struct Comment {
    Element
    content string
}

struct Variable {
    Element
    name string
}
struct VariableFunction {
    Variable
    parameters []ElementGroup
    kw_parameters map[ElementGroup]ElementGroup
}
struct VariableAttribute {
    Variable
    parent ElementGroup
}
struct VariableAttributeFunction {
    VariableAttribute
    VariableFunction
}

struct Statement {
    content []Token
}

type ElementGroup = Element
                  | Comment
                  | Variable | VariableFunction | VariableAttribute | VariableAttributeFunction
                  | Token

fn parse_expression(pre_elements []ElementGroup) []ElementGroup {
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
                    new_elements << parse_expression(catcher).clone()
                    catcher.clear()
                } else {new_elements << selected} // or else, it's function arguments
            } else {new_elements << selected}
        } else {new_elements << selected}
        
        cursor++
    }
    elements = new_elements.clone()
    new_elements.clear()

    // parse variables and function arguments

    println(elements)

    return elements
}

fn parse(preinput []Token) []string {
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
        parse_expression(statement)
    }

    return []
}