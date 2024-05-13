use std::collections::VecDeque;

use crate::{instr::Instr, op};
use thiserror::Error;

#[derive(Debug)]
enum Value {
    Integer(i64),
}

pub struct VM {
    stack: VecDeque<Value>,
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Invalid number of bytes: {0} (must be divisible by 8)")]
    InvalidNumberOfBytes(usize),

    #[error("Stack empty")]
    StackEmpty,

    #[error("Invalid type: Expected {0:?}, got {1:?}")]
    InvalidType(Value, Value),
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<Instr>) -> Result<(), InterpreterError> {
        for instr in bytecode {
            println!(">> stack: (bottom) {:?} (top)", self.stack);
            match instr.op {
                op::Op::Push => {
                    println!("> push {}", instr.value);
                    self.stack.push_back(Value::Integer(instr.value));
                }
                op::Op::Pop => {
                    println!("> pop");
                    _ = self.stack.pop_back();
                }
                op::Op::Print => {
                    println!("> print");
                    if let Some(value) = self.stack.pop_back() {
                        match value {
                            Value::Integer(int) => println!("{}", int),
                        }
                    }
                }
                op::Op::Add => {
                    println!("> add");
                    let b = self.pop_stack_top()?;
                    let a = self.pop_stack_top()?;

                    if let Value::Integer(b) = b {
                        if let Value::Integer(a) = a {
                            self.stack.push_back(Value::Integer(a + b));
                        } else {
                            return Err(InterpreterError::InvalidType(Value::Integer(0), a));
                        }
                    } else {
                        return Err(InterpreterError::InvalidType(Value::Integer(0), a));
                    }
                }
                _ => todo!("Op::{:?}", instr.op),
            }
        }
        Ok(())
    }

    fn pop_stack_top(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop_back().ok_or(InterpreterError::StackEmpty)
    }
}

const INSTR_SIZE: usize = 8;

pub fn decode_instructions(bytes: Vec<u8>) -> anyhow::Result<Vec<Instr>> {
    if bytes.len() % INSTR_SIZE != 0 {
        return Err(InterpreterError::InvalidNumberOfBytes(bytes.len()).into());
    }
    let mut result = Vec::with_capacity(bytes.len() / INSTR_SIZE);

    for n in 0..(bytes.len() / INSTR_SIZE) {
        let mut bytes8 = [0; 8];
        bytes8[0..].copy_from_slice(&bytes[n * INSTR_SIZE..(n + 1) * INSTR_SIZE]);
        result.push(Instr::from_u64(u64::from_be_bytes(bytes8)));
    }

    Ok(result)
}

pub fn encode_instructions(bytecode: &[Instr]) -> anyhow::Result<Vec<u8>> {
    let mut result = Vec::with_capacity(bytecode.len() * INSTR_SIZE);

    for instr in bytecode {
        let encoded = &instr.to_u64();
        for byte in encoded.to_be_bytes() {
            result.push(byte);
        }
    }

    Ok(result)
}
