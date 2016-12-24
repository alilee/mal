use std::io;
use std::io::Write;

extern crate rustali;
use rustali::reader;
use rustali::printer;

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    reader::read_str(line)
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) {
    printer::pr_str()
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
