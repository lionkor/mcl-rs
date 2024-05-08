use crate::{op::Op, instr::Instr};

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
            "push" => Token::Op(Op::Push),
            "pop" => Token::Op(Op::Pop),
            "print" => Token::Op(Op::Print),
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
            [Token::Op(Op::Push), Token::Value(value), rest @ ..] => {
                tail = rest;
                result.push(Instr { op: Op::Push, value: *value })
            }
            [Token::Op(Op::Pop), Token::Value(value), rest @ ..] => {
                tail = rest;
                result.push(Instr { op: Op::Pop, value: *value })
            }
            [Token::Op(Op::Print), rest @ ..] => {
                tail = rest;
                result.push(Instr { op: Op::Print, value: 0 })
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
