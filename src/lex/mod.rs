pub mod constants;
pub mod dfa;

#[derive(Debug)]
pub struct RollResult {
    pub sum: i32,
    pub min: i32,
    pub max: i32,
    pub values: Vec<i32>,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub carry: i32,
    pub result: (i32, i32, i32),
    pub f: i32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            ttype: constants::TOKEN_UNRECSYM,
            lexeme: String::from(""),
            attr: 0,
            carry: 0,
            result: (0, 0, 0),
            f: 0,
        }
    }
}

pub fn nfa(src: &String, pos: i32) -> Token {
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
