use crate::op::Op;

#[derive(Debug)]
pub struct Instr {
    pub op: Op,
    pub value: i64,
}

impl Instr {
    pub fn to_u64(&self) -> u64 {
        // TODO: get the sign bit back!!!
        ((self.op as u64) << 56) | ((self.value as u64) & 0x00FF_FFFF_FFFF_FFFF)
    }
    pub fn from_u64(raw: u64) -> Self {
        Self {
            op: Op::from_repr(((raw & 0xFF00_0000_0000_0000) >> (64 - 8)) as u8).unwrap(),
            // TODO ^ catch and propragate this later
            value: (raw & 0x00FF_FFFF_FFFF_FFFF) as i64,
        }
    }
}
