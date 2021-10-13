[noreturn]
fn error(code string, msg string) {
	println("\033[0;31mError $code: $msg\033[0m")
	exit(1)
}