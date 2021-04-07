pub mod constants;
pub mod dfa;
use std::convert::TryInto;

pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub f: i32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            ttype: constants::TOKEN_UNRECSYM,
            lexeme: String::from(""),
            attr: 0,
            f: 0,
        }
    }
}

pub fn lex(src: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let len: i32 = src.len().try_into().unwrap();
    let mut pos = 0;

    while pos != len + 1 {
        let tok = nfa(pos, src);
        pos = tok.f;

        if tok.ttype != constants::TOKEN_WS {
            tokens.push(tok)
        }
    }
    tokens
}

pub fn nfa(pos: i32, src: &String) -> Token {
    let mut tok = Token::new();
    tok.f = pos;

    dfa::whitespace::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::num::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::d::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::relop::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::extrema::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::fmin::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::fmax::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::fcount::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa::catchall::machine(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    tok.ttype = constants::TOKEN_LEXERR;
    tok.f += 1;
    tok
}
