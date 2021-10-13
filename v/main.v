import os
const version = "0.0.0"

fn compile(input string) []Token {
	return lex(input)
}

fn main() {
	cmd := os.args[1] or {""}
	match cmd {
		'version' {
			println("Zyxt version" + version)
		}
		'run' {
			file := os.args[2] or {error("0.1", "No file given")}
			println(file)
			println(compile(file))
		}
		'compile' {
			println("Coming soon!")
		}
		'interpret' {
			println("Coming soon!")
		}
		else {
			// print help page
		}
	}
}