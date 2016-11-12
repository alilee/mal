use std::io;
use std::io::Write;

fn read() -> String {
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line
}

fn eval(line: String) -> String {
	line
}

fn print(line: String) {
	print!("{}", line);
}

fn print_prompt() {
	print!("{}", "user> ");
	io::stdout().flush().unwrap();
}

fn main() {
	loop {
		print_prompt();
		let line = read();
		let result = eval(line);
		print(result);
	}
}
