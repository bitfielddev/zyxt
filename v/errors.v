[noreturn]
fn error_main(code string, msg string) {
	println(ansi['red']+"Error $code: $msg"+ansi['reset'])
	exit(1)
}

fn error_pos(line int, column int) {
	print(ansi['red']+"($line, $column): "+ansi['reset'])
}

// Internal error
[noreturn]
fn error_0_0(stack string) {
	error_main("0.0", "Internal error: \n$stack")
}

// No file given
[noreturn]
fn error_0_1() {
	error_main("0.1", "No file given")
}

// File does not exist
[noreturn]
fn error_1_0(filename string) {
	error_main("1.0", "File $filename does not exist")
}

// file cannot be opened
[noreturn]
fn error_1_1(filename string) {
	error_main("1.1", "File $filename cannot be opened")
}

// parentheses not closed properly
[noreturn]
fn error_2_0(paren1 string, paren2 string) {
	error_main("2.0", "Parentheses `$paren1` and `$paren2` not closed properly; try swapping them")
}