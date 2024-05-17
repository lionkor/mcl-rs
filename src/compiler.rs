use crate::{instr::Instr, op::Op};

#[derive(Debug)]
enum Token {
    Op(Op),
    Value(i64),
    Unknown(String),
}

fn tokenize(content: &str) -> Result<Vec<Token>, String> {
    let result: Vec<Token> = content
        .trim()
        .split(char::is_whitespace)
        .filter(|raw_token| !raw_token.trim().is_empty())
        .map(|raw_token| match raw_token {
            "pop" => Token::Op(Op::Pop),
            "add" => Token::Op(Op::Add),
            "inc" => Token::Op(Op::Inc),
            "dec" => Token::Op(Op::Dec),
            "sub" => Token::Op(Op::Sub),
            "mul" => Token::Op(Op::Mul),
            "div" => Token::Op(Op::Div),
            "mod" => Token::Op(Op::Mod),
            "print" => Token::Op(Op::Print),
            "halt" => Token::Op(Op::Halt),
            "dup" => Token::Op(Op::Dup),
            // dup2 not callable by the user
            "swap" => Token::Op(Op::Swap),
            "clear" => Token::Op(Op::Clear),
            "over" => Token::Op(Op::Over),
            "push" => Token::Op(Op::Push),
            "je" => Token::Op(Op::Je),
            "jn" => Token::Op(Op::Jn),
            "jg" => Token::Op(Op::Jg),
            "jl" => Token::Op(Op::Jl),
            "jge" => Token::Op(Op::Jge),
            "jle" => Token::Op(Op::Jle),
            "jmp" => Token::Op(Op::Jmp),
            "jz" => Token::Op(Op::Jz),
            "jnz" => Token::Op(Op::Jnz),
            val => {
                if let Ok(int) = i64::from_str_radix(val, 10) {
                    Token::Value(int)
                } else {
                    Token::Unknown(val.to_string())
                }
            }
        })
        .collect();
    for token in &result {
        match token {
            Token::Unknown(val) => return Err(format!("Unexpected token: '{}'", val)),
            _ => (),
        }
    }
    Ok(result)
}

fn compile_to_instrs(tokens: &[Token]) -> Result<Vec<Instr>, String> {
    let mut result: Vec<Instr> = Vec::new();
    let mut tail = tokens;
    loop {
        if tail.is_empty() {
            break;
        }
        match tail {
            [Token::Op(op), rest @ ..] if *op < Op::Push => {
                tail = rest;
                result.push(Instr { op: *op, value: 0 })
            }
            [Token::Op(op), Token::Value(value), rest @ ..] if *op >= Op::Push => {
                tail = rest;
                result.push(Instr {
                    op: *op,
                    value: *value,
                })
            }
            tok => return Err(format!("Invalid token! Expected Op, got '{:?}'", tok)),
        }
    }
    Ok(result)
}

pub fn compile(content: &str) -> Result<Vec<Instr>, String> {
    let tokens = tokenize(content)?;
    println!("{:#?}", tokens);
    let instrs = compile_to_instrs(&tokens)?;
    println!("{:#?}", instrs);
    Ok(instrs)
}
