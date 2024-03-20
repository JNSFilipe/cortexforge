use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fs::{read_to_string, File};
use std::io::{Read, Write};

// TODO: Use wrapping adds and subtractions to prevent overflows

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Operation {
    pub token: Token,
    pub count: u32,
    pub match_addr: u32,
}

// TODO: Put this on the utils file
pub enum Error {
    OutOfBounds,
    InvalidToken,
    EmptyInstructions,
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
        // TODO. Use Result to handle errors, like is done at the at() function
        if self.pointer + 1 >= self.data.len() {
            return 0;
        }
        self.data.as_bytes()[self.pointer + 1]
    }

    fn at(&mut self, index: usize) -> Result<Token, Error> {
        if index >= self.data.len() {
            return Err(Error::OutOfBounds);
        }
        Ok(Token::from(self.data.as_bytes()[index]))
    }

    fn increment(&mut self) {
        self.pointer += 1;
    }
}

#[derive(Clone)]
pub struct IntermRep {
    pub operations: Vec<Operation>,
}
impl std::fmt::Debug for IntermRep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // for operation in &self.operations {
        for i in 0..self.operations.len() {
            let op = &self.operations[i];
            match op.token {
                Token::INC | Token::DEC | Token::LEFT | Token::RIGHT => {
                    writeln!(f, "{}:\t{:?}\t({})", i, op.token, op.count)?;
                }
                Token::LOOP | Token::POOL => {
                    // TODO: Maybe print jump address?
                    writeln!(f, "{}:\t{:?}\t{{{}}}", i, op.token, op.match_addr)?;
                }
                Token::INPUT | Token::OUTPUT => {
                    writeln!(f, "{}:\t{:?}", i, op.token)?;
                }
            }
        }
        Ok(())
    }
}

impl IntermRep {
    pub fn new() -> IntermRep {
        IntermRep {
            operations: Vec::new(),
        }
    }

    pub fn from_source_string(&mut self, data: String) {
        let mut src = Source::new(data);

        let mut operations = Vec::new();
        let mut jump_stack = Vec::new();
        while src.pointer < src.data.len() {
            let token = src.curr();

            match token {
                b'+' | b'-' | b'<' | b'>' | b',' | b'.' => {
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
                    operations.push(Operation {
                        token: Token::from(token),
                        count: 1,
                        match_addr: 1,
                    });

                    jump_stack.push(operations.len() - 1);
                    src.increment();
                }
                b']' => {
                    match jump_stack.pop() {
                        Some(addr) => {
                            operations.push(Operation {
                                token: Token::from(token),
                                count: 1,
                                match_addr: addr as u32,
                            });
                            operations[addr].match_addr = (operations.len() as u32) - 1;
                        }
                        None => panic!("Unmatched brackets at {} (stack empty)", src.pointer),
                    }

                    src.increment();
                }
                _ => {
                    panic!("Unkwon Instruction {}: Something went really wrong", token);
                }
            }
        }

        if !jump_stack.is_empty() {
            panic!("Unmatched brackets at {} (stack not empty)", src.pointer);
        }

        self.operations = operations;
    }

    pub fn to_compiled_file(&self, path: &str) -> Result<(), Error> {
        let mut file = File::create(path).expect("Unable to create file");
        for op in &self.operations {
            match op.token {
                Token::LOOP | Token::POOL => {
                    writeln!(file, "{:?} {}", op.token, op.match_addr)
                        .expect("Unable to write to file");
                }
                _ => {
                    writeln!(file, "{:?} {}", op.token, op.count).expect("Unable to write to file");
                }
            }
        }
        if self.operations.is_empty() {
            return Err(Error::EmptyInstructions);
        }
        Ok(())
    }

    pub fn to_compiled_binary(&self, path: &str) -> Result<(), Error> {
        let encoded: Vec<u8> = bincode::serialize(&self.operations).expect("Unable to serialize");
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(&encoded).expect("Unable to write to file");
        Ok(())
    }

    pub fn from_compiled_file(&mut self, path: &str) -> Result<(), Error> {
        let mut result = Vec::new();

        for line in read_to_string(path).expect("Unable to read file").lines() {
            let parts = line.split(" ").collect::<Vec<&str>>();
            match parts[0] {
                "INC" => {
                    result.push(Operation {
                        token: Token::INC,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "DEC" => {
                    result.push(Operation {
                        token: Token::DEC,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "RIGHT" => {
                    result.push(Operation {
                        token: Token::RIGHT,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "LEFT" => {
                    result.push(Operation {
                        token: Token::LEFT,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "OUTPUT" => {
                    result.push(Operation {
                        token: Token::OUTPUT,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "INPUT" => {
                    result.push(Operation {
                        token: Token::INPUT,
                        count: parts[1].parse().expect("Unable to parse argument"),
                        match_addr: 0,
                    });
                }
                "LOOP" => {
                    result.push(Operation {
                        token: Token::LOOP,
                        count: 1,
                        match_addr: parts[1].parse().expect("Unable to parse argument"),
                    });
                }
                "POOL" => {
                    result.push(Operation {
                        token: Token::POOL,
                        count: 1,
                        match_addr: parts[1].parse().expect("Unable to parse argument"),
                    });
                }
                _ => {
                    panic!("Invalid Token {}", parts[0]);
                }
            }
        }
        self.operations = result;
        Ok(())
    }

    pub fn from_compiled_binary(&mut self, path: &str) -> Result<(), Error> {
        let mut file = File::open(path).expect("Unable to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Unable to read file");
        let operations = bincode::deserialize(&buffer).expect("Unable to deserialize");

        self.operations = operations;
        Ok(())
    }
}
