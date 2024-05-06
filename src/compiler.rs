use crate::op::Op;

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

pub fn compile(content: &str) -> anyhow::Result<Vec<u64>> {
    let tokens = tokenize(content);
    println!("{:#?}", tokens);
    Ok(vec![])
}
