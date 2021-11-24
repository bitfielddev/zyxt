import regex

struct PositionTracker {
mut:
    filename string = "[unknown]"
    line int = 1
    column int = 1
    prev_column int = -1
    char_pos int
}

struct StateTracker {
mut:
    position &PositionTracker
    is_literal_string bool
    literal_string_type TokenType = .null
    prev_type TokenType = .null
    literal_string_line int
    literal_string_column int
    token_line int = 1
    token_column int = 1
    brackets []string
}

fn get_next_char(mut c &string, input string, mut stack []string, mut position &PositionTracker, states StateTracker) ?bool {
    if c == '\n' { // if newline, update line_no
        position.line++
        position.prev_column = position.column
        position.column = 1
    } else {position.column++}
    position.char_pos++
    b := input[position.char_pos] ?
    c = b.ascii_str()
    if (c == " " || c == "\n" || c == "\r") && states.is_literal_string {stack << c}
    else if !(c == " " || c == "\n" || c == "\r") {stack << c}
    return true
}
fn get_next_char_noupdate(input string, position PositionTracker) string {
    b := input[position.char_pos+1] or {byte(0)}
    c := b.ascii_str()
    return c
}

fn get_token_entry(stack []string, states &StateTracker, input string, position PositionTracker) (map[string]TokenEntry) {
    for prevalue, entry in token_catalogue {
        mut value := prevalue
        for value.len != 0 && value[value.len-1].ascii_str() == " " {value = value[..value.len-1]}
        mut re1 := regex.regex_opt(entry.next_prohibited) or {error_0_0(err.msg)}
        mut re2 := regex.regex_opt(entry.prohibited) or {error_0_0(err.msg)}

        if ((!entry.match_whole && stack.join("").ends_with(value))
            || (entry.match_whole && stack.join("") == value)) // if the stack ends with the token tested
           && entry.condition(states) // and the stack satisfies the conditions
           && (entry.next_prohibited.len == 0
               || re1.matches_string(get_next_char_noupdate(input, position))) // and the next character is invalid to be part of the token
           && (entry.prohibited.len == 0 // and the stack itself is valid
               || !re2.matches_string(stack.join("")))
        {
            if value.len == 0 {return {stack.join(""): entry}}
            else {return {value: entry}}
        }
    }
    return {}
}


fn lex(preinput string, filename string) []Token {
    if preinput.trim_space().len == 0 {
        return []Token{}
    }
    input := preinput + "\n"
	mut out := []Token{}
	mut stack := []string{}

    mut position := PositionTracker{filename: filename}
    mut states := StateTracker{position: &position}
    mut c := input[0].ascii_str()
    stack << c

    loop: for {
        if c == '\r' && !states.is_literal_string {
            get_next_char(mut &c, input, mut &stack, mut &position, states) or {break loop}
            continue
        }
        for token, token_entry in get_token_entry(stack, states, input, position) {
            if TokenCategory.literal_string_end in token_entry.categories {
                out << Token{
                    value: stack.join("").substr(0, stack.len-token.len)
                    type_: states.literal_string_type
                    line: states.literal_string_line
                    column: states.literal_string_column
                    categories: [.literal]
                }
                stack.clear()
                stack << token.split("")
                states.literal_string_line = 0
                states.literal_string_column = 0
            } else if TokenCategory.literal_string_start in token_entry.categories {
                states.literal_string_line = position.line
                states.literal_string_column = position.column+1
            }

            token_entry.state_changes(mut states)
            states.prev_type = token_entry.type_

            out << Token{
                value: stack.join("")
                type_: token_entry.type_
                line: position.line
                column: position.column+1-token.len
                categories: token_entry.categories
            }
            stack.clear()
        }

        get_next_char(mut &c, input, mut &stack, mut &position, states) or {break loop}
    }
    if stack.join("").trim_space().len != 0 {
        new_token := Token{
            value: stack.join("")
            type_: .variable
            line: position.line
            column: position.column+1-stack.join("").trim_space().len
            categories: []
        }
        out << new_token
    }


    mut cursor := 0
    mut selected := Token{}
    mut new_out := []Token{}
    // find & form decimal literal numbers
    for cursor < out.len {
        selected = out[cursor]
        if selected.type_ == .dot_opr
           && (cursor != 0 && out[cursor-1].type_ == .literal_number)
           && (cursor != out.len-1 && out[cursor+1].type_ == .literal_number) {
            new_out.delete_last()
            new_out << Token{
                value: out[cursor-1].value + "." + out[cursor+1].value
                type_: .literal_number
                line: out[cursor-1].line
                column: out[cursor-1].column
                categories: [.literal]
            }
            cursor++
        } else {new_out << out[cursor]}
        
        cursor++
    }

    // if states.brackets still has stuff, parens were not closed properly
    if states.brackets.len != 0 {
        error_pos(filename, position.line, position.column)
        error_2_0_1(states.brackets.last())
    }
    
    return new_out
}