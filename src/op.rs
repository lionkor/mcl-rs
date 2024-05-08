use strum::FromRepr;

#[derive(Debug, FromRepr, Copy, Clone)]
#[repr(u8)]
pub enum Op {
    Push = 0x01,
    Pop = 0x02,
    Print = 0x03,
}
