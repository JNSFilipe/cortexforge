use std::collections::HashMap;
use std::convert::From;

// TODO: Save the intermeditate representation to a file
// (do a binary file by default, but allow text, for readability)

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
    match_addr: u32,
}

struct Source {
    data: String,
    pointer: usize,
}

pub enum SegFaultError {
    OutOfBounds,
    InvalidToken,
}

impl Source {
    fn new(data: String) -> Source {
        Source { data, pointer: 0 }
    }

    fn curr(&mut self) -> u8 {
        self.data.as_bytes()[self.pointer]
    }

    fn next(&mut self) -> u8 {
        // TODO. Use Result to handle errors, like is done at the at() function
        if self.pointer + 1 >= self.data.len() {
            return 0;
        }
        self.data.as_bytes()[self.pointer + 1]
    }

    fn at(&mut self, index: usize) -> Result<Token, SegFaultError> {
        if index >= self.data.len() {
            return Err(SegFaultError::OutOfBounds);
        }
        Ok(Token::from(self.data.as_bytes()[index]))
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
                    writeln!(f, "{:?}\t{{{}}}", operation.token, operation.match_addr)?;
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
    //                           <closing_idx, opening_idx>
    let mut back_jump_table = HashMap::<usize, usize>::new();
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
                    match_addr: 0,
                });

                src.increment();
            }
            b'[' => {
                // TODO: Maybe implement this with a forward jump table? Eitherway, this is a bit ugly
                let mut matching: u8 = 1;
                let mut ptr = src.pointer;
                while matching > 0 {
                    ptr += 1;
                    let curr = src.at(ptr);
                    match curr {
                        Ok(c) => match c {
                            Token::LOOP => matching += 1,
                            Token::POOL => matching -= 1,
                            _ => (),
                        },
                        Err(_) => panic!(
                            "Unmatched brackets at {} (failed at foward search)",
                            src.pointer
                        ),
                    }
                }
                operations.push(Operation {
                    token: Token::from(token),
                    count: 1,
                    match_addr: ptr as u32,
                });
                back_jump_table.insert(ptr, src.pointer);
                src.increment();
            }
            b']' => {
                let match_addr = back_jump_table.get(&src.pointer);
                println!("match_addr: {:?}", back_jump_table);
                match match_addr {
                    Some(addr) => {
                        operations.push(Operation {
                            token: Token::from(token),
                            count: 1,
                            match_addr: *addr as u32,
                        });
                    }
                    None => panic!(
                        "Unmatched brackets at {} (not present at jump table)",
                        src.pointer
                    ),
                }

                src.increment();
            }
            b',' | b'.' => {
                operations.push(Operation {
                    token: Token::from(token),
                    count: 1,
                    match_addr: 0,
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
