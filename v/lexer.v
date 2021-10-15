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
    is_literal_string bool
    literal_string_type TokenType = .null
    prev_type TokenType = .null
    literal_string_line int
    literal_string_column int
    token_line int = 1
    token_column int = 1
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
fn get_next_char_noupdate(input string, position PositionTracker) string {
    b := input[position.char_pos+1] or {byte(0)}
    c := b.ascii_str()
    return c
}

fn get_token_entry(stack []string, states &StateTracker) (map[string]TokenEntry) {
    for value, entry in token_catalogue {
        if entry.condition(states) && stack.join("").ends_with(value) {return {value: entry}}
    }
    return {}
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
        if c == '\r' && !states.is_literal_string {continue}
        for token, token_entry in get_token_entry(stack, states) {
            if token_entry.is_literal_string_end {
                lstring := stack.join("").substr(0, stack.len-token.len)
                lstring_token := Token{
                    value: lstring
                    type_: states.literal_string_type
                    line: states.literal_string_line
                    column: states.literal_string_column
                }
                out << lstring_token
                stack.clear()
                stack << token.split("")
                states.literal_string_line = 0
                states.literal_string_column = 0
            } else if token_entry.is_literal_string_start {
                states.literal_string_line = position.line
                states.literal_string_column = position.column+1
            }

            token_entry.state_changes(mut states)
            states.prev_type = token_entry.type_

            new_token := Token{
                value: stack.join("")
                type_: token_entry.type_
                line: position.line
                column: position.column+1-token.len
            }
            out << new_token
            stack.clear()
        }

        get_next_char(mut &c, input, mut &stack, mut &position) or {break loop}
    }
    return out
}