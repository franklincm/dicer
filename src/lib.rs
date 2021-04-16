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

pub fn eval(src: &String) -> Result<EvalResult, EvalError> {
    let mut token: Token = nfa(src, 0);
    let mut output = String::from("");
    parse_expression(&mut token, src, &mut output);

    match token.ttype {
        constants::TOKEN_EOF => Ok(EvalResult {
            value: token.carry,
            str: output,
        }),
        _ => Err(EvalError),
    }
}

pub fn token_type_to_str(ttype: i32) -> String {
    match ttype {
        99 => String::from("TOKEN_LEXERR"),
        0 => String::from("TOKEN_WS"),
        1 => String::from("TOKEN_ADDOP"),
        2 => String::from("TOKEN_MULOP"),
        3 => String::from("TOKEN_COMMA"),
        4 => String::from("TOKEN_LPAREN"),
        5 => String::from("TOKEN_RPAREN"),
        6 => String::from("TOKEN_EOF"),
        7 => String::from("TOKEN_D"),
        8 => String::from("TOKEN_NUM"),
        9 => String::from("TOKEN_RELOP"),
        10 => String::from("TOKEN_EXTREMA"),
        11 => String::from("TOKEN_FMIN"),
        12 => String::from("TOKEN_FMAX"),
        13 => String::from("TOKEN_FCOUNT"),
        101 => String::from("TOKEN_SYNERR"),
        102 => String::from("TOKEN_RBRACKET"),
        103 => String::from("TOKEN_LBRACKET"),
        _ => String::from("TOKEN_UNRECSYM"),
    }
}
