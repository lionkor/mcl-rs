use std::collections::VecDeque;

use crate::{instr::Instr, op};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Value {
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

    #[error("Division by zero")]
    DivByZero,
    
    #[error("Division (modulo) by zero")]
    ModByZero,
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
                    self.transform_top_two_integers(|a, b| Ok(a + b))?;
                }
                op::Op::Inc => {
                    println!("> inc");
                    self.transform_top_integer(|a| *a += 1)?;
                }
                op::Op::Dec => {
                    println!("> dec");
                    self.transform_top_integer(|a| *a -= 1)?;
                }
                op::Op::Sub => {
                    println!("> sub");
                    self.transform_top_two_integers(|a, b| Ok(a - b))?;
                }
                op::Op::Mul => {
                    println!("> mul");
                    self.transform_top_two_integers(|a, b| Ok(a * b))?;
                }
                op::Op::Div => {
                    println!("> div");
                    self.transform_top_two_integers(|a, b| {
                        if b == 0 {
                            Err(InterpreterError::DivByZero)
                        } else {
                            Ok(a / b)
                        }
                    })?;
                }
                op::Op::Mod => {
                    self.transform_top_two_integers(|a, b| {
                        if b == 0 {
                            Err(InterpreterError::ModByZero)
                        } else {
                            Ok(a % b)
                        }
                    })?;
                }
                op::Op::Halt => todo!(),
                op::Op::Dup => todo!(),
                op::Op::Dup2 => todo!(),
                op::Op::Swap => todo!(),
                op::Op::Clear => todo!(),
                op::Op::Over => todo!(),
                op::Op::Je => todo!(),
                op::Op::Jn => todo!(),
                op::Op::Jg => todo!(),
                op::Op::Jl => todo!(),
                op::Op::Jge => todo!(),
                op::Op::Jle => todo!(),
                op::Op::Jmp => todo!(),
                op::Op::Jz => todo!(),
                op::Op::Jnz => todo!(),
                _ => todo!("Op::{:?}", instr.op),
            }
        }
        Ok(())
    }

    fn pop_stack_top(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop_back().ok_or(InterpreterError::StackEmpty)
    }

    /// Transforms the top two integers with the given function,
    /// pushes the result back onto the stack.
    fn transform_top_two_integers(
        &mut self,
        f: impl Fn(i64, i64) -> Result<i64, InterpreterError>,
    ) -> Result<(), InterpreterError> {
        let b = self.pop_stack_top()?;
        let a = self.pop_stack_top()?;
        if let Value::Integer(b) = b {
            if let Value::Integer(a) = a {
                self.stack.push_back(Value::Integer(f(a, b)?));
            } else {
                return Err(InterpreterError::InvalidType(Value::Integer(0), a));
            }
        } else {
            return Err(InterpreterError::InvalidType(Value::Integer(0), a));
        }
        Ok(())
    }

    fn transform_top_integer(
        &mut self,
        f: impl Fn(&mut i64) -> (),
    ) -> Result<(), InterpreterError> {
        if let Some(mut back_value) = self.stack.back_mut() {
            if let Value::Integer(back) = &mut back_value {
                Ok(f(back))
            } else {
                Err(InterpreterError::InvalidType(
                    Value::Integer(0),
                    back_value.clone(),
                ))
            }
        } else {
            return Err(InterpreterError::StackEmpty);
        }
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
