struct Element {
	line int
	column int
}

struct Comment {
	Element
	content string
}

struct Statement {
	content []Token
}

fn parse_expression(tokens []Token) {
	// 
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
	mut token_statements := []Token{}
	mut token_stack := []Token{}
	for token in input {
		if token.type_ == .statement_end {
			out << stack
			stack.clear()
		} else {stack << token}
	}

	// generate an AST for each statement
	for statement in token_statements {
		
	}

	return []
}