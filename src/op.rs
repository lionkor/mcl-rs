use strum::FromRepr;

#[derive(Debug, FromRepr, Copy, Clone)]
#[repr(u8)]
pub enum Op {
    Pop = 0x01,
    Add,
    Inc,
    Dec,
    Sub,
    Mul,
    Div,
    Mod,
    Print,
    Halt,
    Dup,
    Dup2,
    Swap,
    Clear,
    Over,

    // with string argument
    // NONE

    // with argument
    Push,

    // keep these at the end
    Je,
    Jn,
    Jg,
    Jl,
    Jge,
    Jle,
    Jmp,
    // special
    Jz,
    Jnz,
}
