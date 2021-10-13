import os
const version = "0.0.0"

fn main() {
	cmd := os.args[1] or {""}
	match cmd {
		'version' {
			println("Zyxt version" + version)
		}
		'run' {
			file := os.args[2] or {error("0.0", "No file given")}
			println(file)
			//println(compile())
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