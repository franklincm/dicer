pub mod lex;
pub mod parse;

use crate::lex::constants;
use crate::lex::nfa;
use crate::lex::Token;
use crate::parse::rdp::parse_expression;
use std::fmt;

pub struct EvalResult {
    pub value: i32,
    pub str: String,
}

pub struct EvalError;

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something bad happened")
    }
}

impl fmt::Debug for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

/// Simulates rolling dice for the given string
///
/// # Example
///
/// ```
/// use dicer::eval;
/// let expr = String::from("1d20 + 4");
/// let result = eval(&expr).unwrap();
/// println!("value: {}", result.value);
/// println!("intermediate rolls: {}", result.str);
/// ```
pub fn eval(src: &String) -> Result<EvalResult, EvalError> {
    let mut token: Token = nfa(src, 0);

    while token.ttype == constants::TOKEN_WS {
        token = nfa(src, token.f);
    }

    let mut output = String::from("");
    parse_expression(&mut token, src, &mut output);

    println!("eval::token.repeat = {}", token.repeat);
    if token.repeat > 1 {
        let cut: Vec<&str> = src.split("{").collect();
        let expr = cut[0];
        let new_src = String::from(format!("{}{{{}}}", expr, token.repeat - 1).as_str());

        let mut t = nfa(&new_src, 0);
        while t.ttype == constants::TOKEN_WS {
            t = nfa(src, t.f);
        }
        eval(&new_src).unwrap();
    }

    match token.ttype {
        constants::TOKEN_EOF => Ok(EvalResult {
            value: token.carry,
            str: output,
        }),
        _ => Err(EvalError),
    }
}
