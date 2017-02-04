use std::io::{self, Write};

struct Reader {
    counter: i32
}

impl Reader {
    fn next(&mut self) {
        self.counter += 1;
    }

    fn peak(&mut self) {

    }

    fn tokenizer(&mut self, line: &str) {

    }

    fn read_form(&mut self) {

    }
}

fn read_str(line: &str) -> Reader {
    let mut reader = Reader{counter: 0};

    reader.tokenizer(line);
    reader.read_form();

    return reader;
}

fn read(line: &str) {

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

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        if line == "exit\n" {
            break;
        }

        rep(&line.to_string());
    }
}
