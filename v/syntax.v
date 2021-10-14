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
    literal // 3, true, null, etc
	literal_string // "abc" etc
    statement_end // ;
    comment
    variable
	null
}

// TODO make functions for literal_strings
struct TokenEntry {
    type_ TokenType // the type of the token
    isolated bool // whether the token is isolated (idk how explain); eg. * and ** are isolated but // isnt cos you can put anything after it
	condition fn (&StateTracker) bool // conditions needed for the token to be valid; do not change states.prev_type that is already handled by the lexer
	state_changes fn (&StateTracker) // the state changes that are taken place after the token is validated
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
		state_changes: fn(states &StateTracker) bool {
			return states.prev_type != .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = true
			states.literal_string_type = .comment
		}
	}
	"*/": TokenEntry{
		type_: .multiline_comment_end
		condition: fn(states &StateTracker) bool {
			return states.prev_type == .multiline_comment_start
		}
		state_changes: fn (mut states &StateTracker) {
			states.is_literal_string = false
			states.literal_string_type = .null
		}
		is_literal_string_end: true
	}
}