struct Token {
	value string
	type_ TokenType
	line int
	column int
}

struct PositionTracker {
mut:
    line int = 1
    column int = 1
    prev_column int = -1
    char_pos int
}

struct StateTracker {
mut:
    is_literal bool = false
    literal_string_type TokenType = .null
    prev_type TokenType = .null
}

fn get_next_char(mut c &string, input string, mut stack []string, mut position &PositionTracker) ?bool {
    if c == '\n' { // if newline, update line_no
        position.line++
        position.prev_column = position.column
        position.column = 1
    } else {position.column++}
    position.char_pos++
    b := input[position.char_pos] ?
    c = b.ascii_str()
    stack << c
    return true
}

fn lex(preinput string) []Token {
    if preinput.trim_space().len == 0 {
        return []Token{}
    }
    input := preinput + "\n"
	mut out := []Token{}
	mut stack := []string{}

    mut position := PositionTracker{}
    mut states := StateTracker{}
    mut c := input[0].ascii_str()
    stack << c

    loop: for {
        if c == '\r' && states.ignore_whitespace {continue}
        get_next_char(mut &c, input, mut &stack, mut &position) or {break loop}
    }
    println(stack)
    return []
}