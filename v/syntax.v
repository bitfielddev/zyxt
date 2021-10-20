pub enum TokenType {
    comment_start // //
    comment_end // \n
    multiline_comment_start // /*
    multiline_comment_end // */
    flag // hoi, pub, priv, prot, const
    unary_opr // \~, ++, ! etc
    assignment_opr // =, +=, etc
    arithmetic_bitwise_opr // +, -, /f, rt, \& etc
    relational_opr // ==, >, is etc
    logical_opr // &&, ||, ^^ etc
    concat_opr // ..
    swap_opr // ><
    type_opr // istype, isnttype etc
	dot_opr // .
    literal_misc // true, null, etc
	literal_number // 3, 24, -34.5 etc
	literal_string // "abc" etc
    statement_end // ;
	bracket_open // (, [, {
	bracket_close // ), ], }
    comment
    variable
	null
}

// TODO make functions for literal_strings
struct TokenEntry {
    type_ TokenType // the type of the token
	condition fn (&StateTracker) bool = fn (states &StateTracker) bool {
		return !states.is_literal_string
	} // conditions needed for the token to be valid; do not change states.prev_type that is already handled by the lexer
	state_changes fn (&StateTracker) = fn(states &StateTracker) {} // the state changes that are taken place after the token is validated
	prohibited string // the values for the token to be invalid, given as a regex (if the token is a "")
	next_prohibited string // the values for the next character that are invalid, given as a regex of a single character
	match_whole bool // false: only the end of the stack needs to match; true: the entire stack needs to match
	is_literal_string_start bool // if the token marks the start of a literal string.
	is_literal_string_end bool // if the token marks the end of a literal_string.
}

const token_catalogue = {
    "//": TokenEntry{
		type_: .comment_start
		condition: fn (states &StateTracker) bool {
			return states.prev_type != .comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = true
			states.literal_string_type = .comment
		}
		is_literal_string_start: true
	}
	"\n": TokenEntry{
		type_: .comment_end
		condition: fn (states &StateTracker) bool {
			return states.prev_type == .comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = false
			states.literal_string_type = .null
		}
		is_literal_string_end: true
	}
	"/*": TokenEntry{
		type_: .multiline_comment_start
		condition: fn (states &StateTracker) bool {
			return states.prev_type != .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = true
			states.literal_string_type = .comment
		}
		is_literal_string_start: true
	}
	"*/": TokenEntry{
		type_: .multiline_comment_end
		condition: fn (states &StateTracker) bool {
			return states.prev_type == .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = false
			states.literal_string_type = .null
		}
		is_literal_string_end: true
	}
	"+": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^+-=]"
	}
	"-": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^+-=]"
	}
	"+-": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"-+": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"±": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"∓": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"·": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"*": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=/]"
	}
	"×": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"/": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^fc~=*/]"
	}
	"÷": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^fc~=]"
	}
	"/f": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"/c": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"/~": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"÷f": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"÷c": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"÷~": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"^": TokenEntry{
		type_: .arithmetic_bitwise_opr
	}
	"%": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"rt": TokenEntry{
		type_: .arithmetic_bitwise_opr
		match_whole: true
	}
	"lg": TokenEntry{
		type_: .arithmetic_bitwise_opr
		match_whole: true
	}
	"\\&": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"\\|": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"\\^": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"\\<<": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"\\>>": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=>]"
	}
	"\\>>>": TokenEntry{
		type_: .arithmetic_bitwise_opr
		next_prohibited: r"[^=]"
	}
	"=": TokenEntry{
		type_: .assignment_opr
		next_prohibited: r"[^=]"
	}
	"+=": TokenEntry{
		type_: .assignment_opr
	}
	"-=": TokenEntry{
		type_: .assignment_opr
	}
	"*=": TokenEntry{
		type_: .assignment_opr
	}
	"/=": TokenEntry{
		type_: .assignment_opr
	}
	"/f=": TokenEntry{
		type_: .assignment_opr
	}
	"/c=": TokenEntry{
		type_: .assignment_opr
	}
	"/~=": TokenEntry{
		type_: .assignment_opr
	}
	"%=": TokenEntry{
		type_: .assignment_opr
	}
	"\\&=": TokenEntry{
		type_: .assignment_opr
	}
	"\\|=": TokenEntry{
		type_: .assignment_opr
	}
	"\\^=": TokenEntry{
		type_: .assignment_opr
	}
	"\\<<=": TokenEntry{
		type_: .assignment_opr
	}
	"\\>>=": TokenEntry{
		type_: .assignment_opr
	}
	"\\>>>=": TokenEntry{
		type_: .assignment_opr
	}
	"==": TokenEntry{
		type_: .relational_opr
		next_prohibited: r"[^=]"
	}
	">": TokenEntry{
		type_: .relational_opr
		next_prohibited: r"[^=<]"
	}
	"<": TokenEntry{
		type_: .relational_opr
		next_prohibited: r"[^=]"
	}
	">=": TokenEntry{
		type_: .relational_opr
	}
	"<=": TokenEntry{
		type_: .relational_opr
	}
	"!=": TokenEntry{
		type_: .relational_opr
		next_prohibited: r"[^=]"
	}
	"===": TokenEntry{
		type_: .relational_opr
	}
	"!==": TokenEntry{
		type_: .relational_opr
	}
	"is": TokenEntry{
		type_: .relational_opr
		match_whole: true
		next_prohibited: "[^tn]"
	}
	"isnt": TokenEntry{
		type_: .relational_opr
		match_whole: true
		next_prohibited: "[^t]"
	}
	"&&": TokenEntry{
		type_: .logical_opr
	}
	"||": TokenEntry{
		type_: .logical_opr
	}
	"^^": TokenEntry{
		type_: .logical_opr
	}
	"istype": TokenEntry{
		type_: .type_opr
		match_whole: true
	}
	"isnttype": TokenEntry{
		type_: .type_opr
		match_whole: true
	}
	"><": TokenEntry{
		type_: .swap_opr
	}
	"..": TokenEntry{
		type_: .concat_opr
	}
	"++": TokenEntry{
		type_: .unary_opr
	}
	"--": TokenEntry{
		type_: .unary_opr
	}
	"\\~": TokenEntry{
		type_: .unary_opr
	}
	"!": TokenEntry{
		type_: .unary_opr
		next_prohibited: r"[^=]"
	}
	"(": TokenEntry{
		type_: .bracket_open
		state_changes: fn (mut states &StateTracker) {
			states.brackets << "("
		}
	}
	"[": TokenEntry{
		type_: .bracket_open
		state_changes: fn (mut states &StateTracker) {
			states.brackets << "["
		}
	}
	"{": TokenEntry{
		type_: .bracket_open
		state_changes: fn (mut states &StateTracker) {
			states.brackets << "{"
		}
	}
	")": TokenEntry{
		type_: .bracket_close
		state_changes: fn (mut states &StateTracker) {
			if states.brackets.last() != "(" {
				error_pos(states.position.line, states.position.column)
				error_2_0(")", states.brackets.last())
			}
			states.brackets.delete_last()
		}
	}
	"]": TokenEntry{
		type_: .bracket_close
		state_changes: fn (mut states &StateTracker) {
			if states.brackets.last() != "[" {
				error_pos(states.position.line, states.position.column)
				error_2_0("]", states.brackets.last())
			}
			states.brackets.delete_last()
		}
	}
	"}": TokenEntry{
		type_: .bracket_close
		state_changes: fn (mut states &StateTracker) {
			if states.brackets.last() != "{" {
				error_pos(states.position.line, states.position.column)
				error_2_0("}", states.brackets.last())
			}
			states.brackets.delete_last()
		}
	}
	".": TokenEntry{
		type_: .dot_opr
		next_prohibited: r"[^\.]"
	}
	"hoi": TokenEntry{
		type_: .flag
		next_prohibited: r"\s"
	}
	"pub": TokenEntry{
		type_: .flag
		next_prohibited: r"\s"
	}
	"priv": TokenEntry{
		type_: .flag
		next_prohibited: r"\s"
	}
	"prot": TokenEntry{
		type_: .flag
		next_prohibited: r"\s"
	}
	"const": TokenEntry{
		type_: .flag
		next_prohibited: r"\s"
	}
	"true": TokenEntry{
		type_: .literal_misc
		next_prohibited: r"\s"
	}
	"false": TokenEntry{
		type_: .literal_misc
		next_prohibited: r"\s"
	}
	"null": TokenEntry{
		type_: .literal_misc
		next_prohibited: r"\s"
	}
	"inf": TokenEntry{
		type_: .literal_misc
		next_prohibited: r"\s"
	}
	";": TokenEntry{
		type_: .statement_end
	}
	"": TokenEntry{
		type_: .literal_number
		prohibited: r"\D"
		next_prohibited: r"\d"
	}
	" ": TokenEntry{
		type_: .variable
		prohibited: r"\W"
		next_prohibited: r"[\W\s]"
	}
}