use crate::instr::Instr;
use thiserror::Error;

enum Value {
    Integer(i64),
}

struct Stack {
    data: Vec<Value>,
}

pub struct VM {
    stack: Stack,
    bytecode: Vec<u64>,
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Invalid number of bytes: {0} (must be divisible by 8)")]
    InvalidNumberOfBytes(usize),
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Stack { data: Vec::new() },
            bytecode: vec![],
        }
    }

    pub fn execute(mut self, bytecode: Vec<Instr>) -> anyhow::Result<()> {
        for instr in bytecode {
            match instr.op {
                crate::op::Op::Push => {
                    println!("push {}", instr.value);
                }
                crate::op::Op::Pop => {
                    println!("pop {}", instr.value);
                }
                crate::op::Op::Print => {
                    println!("print");
                }
                _ => panic!("Invalid instruction!"),
            }
        }
        Ok(())
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
