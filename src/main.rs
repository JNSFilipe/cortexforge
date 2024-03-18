mod utils;
use std::convert::From;
use utils::{filter_chars, read_file};

#[repr(u8)]
#[derive(Debug)]
enum Token {
    INC = b'+',
    DEC = b'-',
    LEFT = b'<',
    RIGHT = b'>',
    LOOP = b'[',
    POOL = b']',
    INPUT = b',',
    OUTPUT = b'.',
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b'+' => Token::INC,
            b'-' => Token::DEC,
            b'<' => Token::LEFT,
            b'>' => Token::RIGHT,
            b'[' => Token::LOOP,
            b']' => Token::POOL,
            b',' => Token::INPUT,
            b'.' => Token::OUTPUT,
            _ => panic!("Invalid token"),
        }
    }
}

struct Operation {
    token: Token,
    count: u32,
}

struct Source {
    data: String,
    pointer: usize,
}

impl Source {
    fn new(data: String) -> Source {
        Source { data, pointer: 0 }
    }

    fn curr(&mut self) -> u8 {
        self.data.as_bytes()[self.pointer]
    }

    fn next(&mut self) -> u8 {
        if self.pointer + 1 >= self.data.len() {
            return 0;
        }
        self.data.as_bytes()[self.pointer + 1]
    }

    fn increment(&mut self) {
        self.pointer += 1;
    }
}

struct Program {
    operations: Vec<Operation>,
}
impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for operation in &self.operations {
            writeln!(f, "{:?}\t({})", operation.token, operation.count)?;
        }
        Ok(())
    }
}

fn parser(src: &mut Source) -> Program {
    let mut operations = Vec::new();
    while src.pointer < src.data.len() {
        let token = src.curr();

        match token {
            b'+' | b'-' | b'<' | b'>' | b'[' | b']' | b',' | b'.' => {
                let mut count = 1;
                while src.next() == token {
                    count += 1;
                    src.increment();
                }

                operations.push(Operation {
                    token: Token::from(token),
                    count: count,
                });

                src.increment();
            }
            _ => {
                src.increment();
            }
        }
    }
    Program { operations }
}

fn main() {
    let file = "hello_world.bf";
    let mut data = read_file(file);
    data = filter_chars(&data);
    let mut src = Source::new(data.clone());

    let program = parser(&mut src);

    println!("\n{}\n\n", data);
    println!("{:?}", program);
}
