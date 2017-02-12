#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate readline;

use std::rc::Rc;
use std::io::{self, Write};
use std::option::Option;
use regex::Regex;

enum LType {
    Nil,
    True,
    False,
    Int(isize),
    Symbol(String),
    List(Vec<Rc<LType>>, Rc<LType>)
}

struct Reader<'a> {
    position: usize,
    tokens: Vec<&'a str>
}

impl<'a> Reader<'a> {
    fn new() -> Reader<'a> {
        Reader{
            position: 0,
            tokens: Vec::new()
        }
    }

    fn next(&mut self) -> Option<&str> {
        if self.position < self.tokens.len() {
            self.position += 1;

            Some(self.tokens.get(self.position - 1).unwrap())
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&str> {
        if self.position < self.tokens.len() {
            Some(self.tokens.get(self.position).unwrap())
        } else {
            None
        }
    }

    fn tokenizer(&mut self, line: &'a str) {
        // [\s,]*               Whitespaces or commas, ignore this
        // ~@                   Special characters ~@
        // [\[\]{}()'`~^@]      Single special character
        // "(?:\\.|[^\\"])*"    From the first : to the next : ignoring \:
        // ;.*                  Sequences of comments ;, ignore this
        // [^\s\[\]{}('"`,;)]*  Sequences of normal characters
        lazy_static! {
            static ref RE:Regex = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
        }

        for cap in RE.captures_iter(line) {
            self.tokens.push(cap.get(1).unwrap().as_str());
            println!("Regex match: {}", cap.get(1).unwrap().as_str());
        }
    }

    fn read_seq(&'a mut self, start: &str, end: &str) -> Result<Vec<Rc<LType>>, String> {
        let token = self.next().unwrap();

        let mut ast_vec: Vec<Rc<LType>> = vec![];
        loop {
            let otoken = self.peek();
            if otoken.is_none() {
                return Err(format!("Expected '{}', got EOF", end));
            }
            let token = otoken.unwrap();
            if token == end {
                break;
            }

            match self.read_form() {
                Ok(lval) => ast_vec.push(Rc::new(lval)),
                Err(err) => return Err(err)
            }
        }

        self.next();

        Ok(ast_vec)
    }

    fn read_form(&'a mut self) -> Result<LType, String> {
        let token = self.peek().unwrap();

        let first_char = token.chars().nth(0).unwrap();

        match first_char {
            '(' => {
                self.read_list();
            },
            ')' => return Err("Found closing bracket".to_string()),

            _ => {
                self.read_atom();
            }
        }

        Ok(LType::Symbol(token.to_string()))
    }

    fn read_list(&'a mut self) -> Result<LType, String> {
        match self.read_seq("(", ")") {
            Ok(seq) => Ok(LType::List(seq, Rc::new(LType::Nil))),
            Err(e) => Err(e)
        }
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

fn read(line: &str) -> &str {
    read_str(line);

    line
}

fn eval(ast: &str) -> &str {
    ast
}

fn print(exp: &str) -> &str {
    exp
}

fn rep(line: &str) {
    print(eval(read(line)));
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
