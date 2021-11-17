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
    parent &Variable
}
struct VariableFunction {
    Variable
    parameters []Element
}

struct Statement {
    content []Token
}

interface ElementGroup {}

fn parse_expression(tokens []Token) {
    mut cursor := 0
    mut cursor_end := -1
    mut selected := Token{}
    mut new_tokens := []ElementGroup{} 

    // parse functions and ()s
    for cursor < tokens.len {
        selected = tokens[cursor]
        if selected.type_ == .dot_opr {
            
        }
        cursor++
    }
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
    mut token_statements := [][]Token{}
    mut token_stack := []Token{}
    for token in input {
        if token.type_ == .statement_end {
            token_statements << token_stack
            token_stack.clear()
        } else {token_stack << token}
    }

    // generate an AST for each statement
    for statement in token_statements {
        parse_expression(statement)
    }

    return []
}