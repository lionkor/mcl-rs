use std::collections::HashMap;

use crate::{instr::Instr, op::Op};

#[derive(Debug)]
enum Token {
    Op(Op),
    Value(i64),
    Label(String),
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
                if let Ok(int) = val.parse::<i64>() {
                    Token::Value(int)
                } else if val.starts_with(':') {
                    // label
                    Token::Label(val.strip_prefix(':').unwrap().to_string())
                } else {
                    Token::Unknown(val.to_string())
                }
            }
        })
        .collect();
    for token in &result {
        if let Token::Unknown(val) = token {
            return Err(format!("Unexpected token: '{}'", val));
        }
    }
    Ok(result)
}

#[derive(Debug)]
enum AbstractValue {
    None,
    Integer(i64),
    Label(String),
}

#[derive(Debug)]
struct AbstractInstr {
    op: Op,
    value: AbstractValue,
}

fn compile_to_instrs(tokens: &[Token]) -> Result<Vec<Instr>, String> {
    let mut abstr_result: Vec<AbstractInstr> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut tail = tokens;
    loop {
        if tail.is_empty() {
            break;
        }
        match tail {
            [Token::Label(name), rest @ ..] => {
                tail = rest;
                if labels.contains_key(name) {
                    return Err(format!("Label '{}' defined more than once", name));
                }
                // insert (name, address of next instr)
                labels.insert(name.clone(), abstr_result.len());
            }
            [Token::Op(op), rest @ ..] if *op < Op::Push => {
                tail = rest;
                abstr_result.push(AbstractInstr {
                    op: *op,
                    value: AbstractValue::None,
                })
            }
            // anything with argument
            [Token::Op(op), Token::Value(value), rest @ ..] if *op >= Op::Push => {
                tail = rest;
                abstr_result.push(AbstractInstr {
                    op: *op,
                    value: AbstractValue::Integer(*value),
                })
            }
            // jumps
            [Token::Op(op), Token::Label(value), rest @ ..] if *op > Op::Push => {
                tail = rest;
                abstr_result.push(AbstractInstr {
                    op: *op,
                    value: AbstractValue::Label(value.clone()),
                })
            }
            tok => return Err(format!("Invalid token! Expected Op, got '{:?}'", tok)),
        }
    }
    println!("{:#?}", abstr_result);
    // resolve labels
    for instr in &mut abstr_result {
        if let AbstractInstr {
            op: _,
            value: AbstractValue::Label(name),
        } = instr
        {
            if labels.contains_key(name) {
                instr.value = AbstractValue::Integer(*labels.get(name).unwrap() as i64);
            } else {
                return Err(format!("Label '{}' is not defined", name));
            }
        }
    }
    println!("{:#?}", abstr_result);
    // concretize to real [`Instr`]
    let result = abstr_result.iter().map(|abstr_instr| {
        Instr { op: abstr_instr.op, value: match abstr_instr.value {
            AbstractValue::Integer(int) => int,
            AbstractValue::None => 0,
            _ => {
                panic!("Should never happen: Non-abstract value in concretization step")
            }
        } }
    }).collect();
    Ok(result)
}

pub fn compile(content: &str) -> Result<Vec<Instr>, String> {
    let tokens = tokenize(content)?;
    println!("{:#?}", tokens);
    let instrs = compile_to_instrs(&tokens)?;
    println!("{:#?}", instrs);
    Ok(instrs)
}
