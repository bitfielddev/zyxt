enum TokenType {
    comment_start // //
    comment_end // \n
    multiline_comment_start // /*
    multiline_comment_end // */
    flag // :
    flag_label // the stuff after :
    unary_opr // b~, ++, ! etc
    assignment_opr // =, +=, etc
    arithmetic_bitwise_opr // +, -, /f, rt, b& etc
    relational_opr // ==, >, is etc
    logical_opr // &&, ||, ^^ etc
    concat_opr // ..
    swap_opr // ><
    type_opr // istype, isnttype et
    literal // "abc", 3, true, null, etc
    statement_end // ;
    comment
    variable
}

struct Token {
	value string
	type_ TokenType
	line_no int
	column_no int
}

fn lex(input string) []Token {
	mut out := []Token{}
	mut stack := []string{}
	return [Token{
		"test",
		.comment,
		1,
		1
	}]
}