pub mod lex;
pub mod parse;

use crate::lex::constants;
use crate::lex::nfa;
use crate::lex::Token;
use crate::parse::rdp::parse_expression;
use std::fmt;

#[derive(Debug, Clone)]
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
/// let results = eval(&expr).unwrap();
/// for res in  results {
///     println!("{} = {}", res.str, res.value);
/// }
/// ```
pub fn eval(src: &str) -> Result<Vec<EvalResult>, EvalError> {
    let mut results: Vec<Result<EvalResult, EvalError>> = Vec::new();
    repeat_eval(src, &mut results);

    let eval_results: Result<Vec<EvalResult>, EvalError> = results.into_iter().collect();

    eval_results
}

fn repeat_eval(src: &str, results: &mut Vec<Result<EvalResult, EvalError>>) {
    let mut token: Token = nfa(src, 0);

    while token.ttype == constants::TOKEN_WS {
        token = nfa(src, token.f);
    }

    let mut output = String::from("");
    parse_expression(&mut token, src, &mut output);

    if token.ttype == constants::TOKEN_EOF {
        results.push(Ok(EvalResult {
            value: token.carry,
            str: output,
        }));
    } else {
        results.push(Err(EvalError));
    }

    if token.repeat > 1 {
        let cut: Vec<&str> = src.split('{').collect();
        let expr = cut[0];
        let new_src = String::from(format!("{}{{{}}}", expr, token.repeat - 1).as_str());

        let mut t = nfa(&new_src, 0);
        while t.ttype == constants::TOKEN_WS {
            t = nfa(src, t.f);
        }
        repeat_eval(&new_src, results);
    }
}
