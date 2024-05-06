
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

    pub fn execute(mut self, bytecode: Vec<u64>) -> anyhow::Result<()> {
        for raw_instr in bytecode {
            let op = (raw_instr & 0xFF00_0000_0000_0000) >> (64 - 8);
            let value = raw_instr & 0x00FF_FFFF_FFFF_FFFF;
            match op {
                1 => {
                    println!("push {}", value);
                }
                _ => panic!("Invalid instruction!"),
            }
        }
        Ok(())
    }

}
