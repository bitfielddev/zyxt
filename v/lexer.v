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
	line int
	column int
}

fn lex(mut input string) []Token {
    if input.trim_space().len == 0 {
        return []Token{}
    }
    input += "\n"
	mut out := []Token{}
	mut stack := []string{}

    mut line_no := 1
    mut column_no := 1
    mut prev_column_no := 1
    mut states := {
        "ignore_whitespace": false
    }
    mut char_pos := 0
    mut c := char_pos[0]

    fn update_position(mut c ?&string) {
        if c == '\n' { // if newline, update lineNo
            line_no++
            prev_column_no = column_no
            column_no = 1
        } else columnNo++
        char_pos++
        c = input[char_pos] ?
    }

    loop: for {
        if (c == '\r' && states['ignore_whitespace']) continue
        update_position(&c) or {break loop}
    }
}