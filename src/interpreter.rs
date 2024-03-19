use crate::ir::{IntermRep, Token};

const MAX_MEMORY: usize = 30000;

// TODO: Support i32 for memory cells
// TODO: Check for memory overflows

pub struct Interpreter {
    ir: IntermRep,
    memory: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    pub fn new(ir: IntermRep) -> Interpreter {
        Interpreter {
            ir,
            memory: vec![0; MAX_MEMORY],
            pointer: 0,
        }
    }

    pub fn run(&mut self) {
        let mut op_pointer = 0;
        while op_pointer < self.ir.operations.len() {
            let op = &self.ir.operations[op_pointer];
            match op.token {
                Token::INC => {
                    self.memory[self.pointer] =
                        self.memory[self.pointer].wrapping_add(op.count as u8);
                }
                Token::DEC => {
                    self.memory[self.pointer] =
                        self.memory[self.pointer].wrapping_sub(op.count as u8);
                }
                Token::LEFT => {
                    self.pointer = self.pointer.wrapping_sub(op.count as usize);
                }
                Token::RIGHT => {
                    self.pointer = self.pointer.wrapping_add(op.count as usize);
                }
                Token::LOOP => {
                    if self.memory[self.pointer] == 0 {
                        op_pointer = op.match_addr as usize;
                    }
                }
                Token::POOL => {
                    if self.memory[self.pointer] != 0 {
                        op_pointer = op.match_addr as usize;
                    }
                }
                Token::OUTPUT => {
                    for _ in 0..op.count {
                        print!("{}", self.memory[self.pointer] as char);
                    }
                }
                Token::INPUT => {
                    for _ in 0..op.count {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        self.memory[self.pointer] = input.as_bytes()[0];
                    }
                }
            }
            op_pointer += 1;
        }
    }
}
