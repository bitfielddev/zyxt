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

fn parse_expression(pretokens []Token) {
    mut cursor := 0
    mut cursor_end := -1
    mut selected := Token{}
    mut tokens := pretokens.clone()
    mut new_tokens := []Token{}

    // find & form 
    for cursor < tokens.len {
        selected = tokens[cursor]
        if selected.type_ == .dot_opr
           && (cursor != 0 && tokens[cursor-1].type_ == .literal_number)
           && (cursor != tokens.len-1 && tokens[cursor+1].type_ == .literal_number) {
            new_tokens.delete_last()
            new_tokens << Token{
                value: tokens[cursor-1].value + "." + tokens[cursor+1].value
                type_: .literal_number
                line: tokens[cursor-1].line
                column: tokens[cursor-1].column
            }
            cursor++
        } else {new_tokens << tokens[cursor]}
        
        cursor++
    }
    tokens = new_tokens.clone()
    new_tokens.clear()
    // parse functions and ()s
    println(tokens)
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
            token_statements << token_stack.clone()
            token_stack.clear()
        } else {token_stack << token}
    }

    // generate an AST for each statement
    for statement in token_statements {
        parse_expression(statement)
    }

    return []
}