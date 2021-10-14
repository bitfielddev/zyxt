const ansi = {
	"reset": "\033[0m"
	"black": "\033[0;30m"
	"red": "\033[0;31m"
	"green": "\033[0;32m"
	"yellow": "\033[0;33m"
	"blue": "\033[0;34m"
	"purple": "\033[0;35m"
	"cyan": "\033[0;36m"
	"white": "\033[0;37m"
}

[noreturn]
fn error(code string, msg string) {
	println(ansi['red']+"Error $code: $msg"+ansi['reset'])
	exit(1)
}

// Internal error
[noreturn]
fn error_0_0(stack string) {
	error("0.0", "Internal error: \n$stack")
}

// No file given
[noreturn]
fn error_0_1() {
	error("0.1", "No file given")
}

// File does not exist
[noreturn]
fn error_1_0(filename string) {
	error("1.1", "File $filename does not exist")
}

// file cannot be opened
[noreturn]
fn error_1_1()