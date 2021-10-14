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

struct PositionTracker {
    line int = 1
    column int = 1
    prev_column int = -1
    char_pos int = 1
}

struct StageTracker {
    ignore_whitespace bool = false
}

fn get_next_char(mut c &string, mut input &string, mut position ) {
    if c == '\n' { // if newline, update line_no
        line_no++
        prev_column_no = column_no
        column_no = 1
    } else {column_no++}
    char_pos++
    c = input[char_pos] ?
}

fn lex(preinput string) []Token {
    if preinput.trim_space().len == 0 {
        return []Token{}
    }
    input := preinput + "\n"
	mut out := []Token{}
	mut stack := []string{}

    mut position := PositionTracker{}
    mut states := StageTracker{}
    mut c := input[0]

    loop: for {
        if (c == '\r' && states.ignore_whitespace) {continue}
        get_next_char(&c, &input, &line_no, &column_no, &prev_column_no) or {break loop}
        out << c
    }
    println(out)
    return []
}