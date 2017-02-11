#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate readline;

use std::io::{self, Write};
use std::option::Option;
use regex::Regex;

struct Reader<'a> {
    counter: usize,
    tokens: Vec<&'a str>
}

impl<'a> Reader<'a> {
    fn new() -> Reader<'a> {
        Reader{
            counter: 0,
            tokens: Vec::new()
        }
    }

    fn next(&mut self) -> &'a str {
        self.counter += 1;

        self.tokens.get(self.counter - 1).unwrap()
    }

    fn peak(&self) -> &'a str {
        self.tokens.get(self.counter).unwrap()
    }

    fn tokenizer(&mut self, line: &'a str) {
        // [\s,]*               Whitespaces or commas, ignore this
        // ~@                   Special characters ~@
        // [\[\]{}()'`~^@]      Single special character
        // "(?:\\.|[^\\"])*"    From the first : to the next : ignoring \:
        // ;.*                  Sequences of comments ;, ignore this
        // [^\s\[\]{}('"`,;)]*  Sequences of normal characters
        lazy_static! {
            static ref re:Regex = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
        }

        for cap in re.captures_iter(line) {
            self.tokens.push(cap.get(1).unwrap().as_str());
            println!("Regex match: {}", cap.get(1).unwrap().as_str());
        }
    }

    fn read_form(&mut self) -> &'a str {
        let peak = self.peak();

        match peak {
            "(" => self.read_list(),
            _ => self.read_atom()
        }

        peak
    }

    fn read_list(&mut self) {

    }

    fn read_atom(&mut self) {

    }
}

fn read_str(line: &str) -> Reader {
    let mut reader = Reader::new();

    reader.tokenizer(line);
    reader.read_form();

    reader
}

fn read(line: &str) {
    read_str(line);
}

fn eval() {

}

fn print() {

}

fn rep(line: &str) {
    read(line);
    eval();
    print();
}

fn main() {
    println!("Welcome to a lisp REPL.");

    loop {
        print!("Λ> ");
        io::stdout().flush().unwrap();

        let input = readline::readline("").unwrap();
        readline::add_history(&input.to_string());

        if input == "exit" {
            break;
        }

        rep(&input.to_string());
    }
}
