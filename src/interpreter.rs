use crate::instr::Instr;


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

impl VM {

    pub fn new() -> Self {
        Self { stack: Stack { data: Vec::new() }, bytecode: vec![] }
    }

    pub fn execute(mut self, bytecode: Vec<Instr>) -> anyhow::Result<()> {
        for instr in bytecode {
            match instr.op {
                crate::op::Op::Push => {
                    println!("push {}", instr.value);
                },
                crate::op::Op::Pop => {
                    println!("pop {}", instr.value);
                },
                crate::op::Op::Print => {
                    println!("print");
                },
                _ => panic!("Invalid instruction!"),
            }
        }
        Ok(())
    }

}
