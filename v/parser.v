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
    datatype string
}
struct VariableFunction {
    Variable
    parameters []Element
    datatype string = "func"
}
struct VariableAttribute {
    Variable
    parent &Variable
}
struct VariableAttributeFunction {
    VariableAttribute
    VariableFunction
}

struct Statement {
    content []Token
}

type ElementToken = Element
                  | Comment
                  | Variable | VariableFunction | VariableAttribute | VariableAttributeFunction
                  | Token

fn parse_expression(pretokens []ElementToken) []ElementToken {
    mut cursor := 0
    mut selected := ElementToken(Token{})
    mut tokens := pretokens.clone()
    mut new_tokens := []ElementToken{}


    tokens = new_tokens.clone()
    new_tokens.clear()
    // parse ()s
    println(tokens) 

    return []ElementToken{}
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
    mut token_statements := [][]ElementToken{}
    mut token_stack := []ElementToken{}
    for token in input {
        if token.type_ == .statement_end {
            token_statements << token_stack.clone()
            token_stack.clear()
        } else {token_stack << ElementToken(token)}
    }

    // generate an AST for each statement
    for statement in token_statements {
        parse_expression(statement)
    }

    return []
}