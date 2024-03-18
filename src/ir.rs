use std::convert::From;

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

pub struct IntermRep {
    operations: Vec<Operation>,
}
impl std::fmt::Debug for IntermRep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for operation in &self.operations {
            match operation.token {
                Token::INC | Token::DEC | Token::LEFT | Token::RIGHT => {
                    writeln!(f, "{:?}\t({})", operation.token, operation.count)?;
                }
                Token::LOOP | Token::POOL => {
                    // TODO: Maybe print jump address?
                    writeln!(f, "{:?}", operation.token)?;
                }
                Token::INPUT | Token::OUTPUT => {
                    writeln!(f, "{:?}", operation.token)?;
                }
            }
        }
        Ok(())
    }
}

fn parser(src: &mut Source) -> IntermRep {
    let mut operations = Vec::new();
    while src.pointer < src.data.len() {
        let token = src.curr();

        match token {
            b'+' | b'-' | b'<' | b'>' => {
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
            b'[' | b']' => {
                // TODO: Must find closing bracket in here
                operations.push(Operation {
                    token: Token::from(token),
                    count: 1,
                });

                src.increment();
            }
            b',' | b'.' => {
                operations.push(Operation {
                    token: Token::from(token),
                    count: 1,
                });

                src.increment();
            }
            _ => {
                src.increment();
            }
        }
    }
    IntermRep { operations }
}

pub fn str_to_ir(data: String) -> IntermRep {
    let mut src = Source::new(data);
    parser(&mut src)
}
