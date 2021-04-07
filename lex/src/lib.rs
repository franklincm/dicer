pub mod constants;
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

pub fn nfa(pos: i32, src: &String) -> Token {
    let mut tok = Token::new();
    tok.f = pos;

    dfa_whitespace(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_num(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_d(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_relop(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_extrema(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_fmin(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_fmax(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_fcount(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    dfa_catchall(&mut tok, src);
    if tok.ttype != constants::TOKEN_UNRECSYM {
        return tok;
    }

    tok.ttype = constants::TOKEN_LEXERR;
    tok.f += 1;
    tok
}

pub fn dfa_whitespace(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    while k < len && &src.chars().nth(k.try_into().unwrap()).unwrap() == &' ' {
        k += 1;
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_WS;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_catchall(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    } else if k == len {
        tok.ttype = constants::TOKEN_EOF;
        tok.f += 1;
        return;
    }

    let char = &src.chars().nth(k.try_into().unwrap()).unwrap();

    if char == &'*' || char == &'/' {
        k += 1;
        tok.ttype = constants::TOKEN_MULOP;
    } else if char == &'+' || char == &'-' {
        k += 1;
        tok.ttype = constants::TOKEN_ADDOP;
    } else if char == &',' {
        k += 1;
        tok.ttype = constants::TOKEN_COMMA;
    } else if char == &'(' {
        k += 1;
        tok.ttype = constants::TOKEN_LPAREN;
    } else if char == &')' {
        k += 1;
        tok.ttype = constants::TOKEN_RPAREN;
    }

    if k > tok.f {
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_num(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    if src.chars().nth(0.try_into().unwrap()).unwrap() == '0' {
        tok.ttype = constants::TOKEN_LEXERR;
        return;
    }

    while k < len && src.chars().nth(k.try_into().unwrap()).unwrap().is_digit(10) {
        k += 1;
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_NUM;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_d(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    match src.chars().nth(k.try_into().unwrap()) {
        Some('d') => k += 1,
        Some('D') => k += 1,
        _ => return,
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_D;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_relop(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    }

    match src.chars().nth(k.try_into().unwrap()) {
        Some('<') => {
            k += 1;
            match src.chars().nth(k.try_into().unwrap()) {
                Some('=') => k += 1,
                _ => (),
            };
        }
        Some('>') => {
            k += 1;
            match src.chars().nth(k.try_into().unwrap()) {
                Some('=') => k += 1,
                _ => (),
            };
        }
        _ => return,
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_RELOP;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

pub fn dfa_extrema(tok: &mut Token, src: &String) {
    let len: i32 = src.len().try_into().unwrap();

    if tok.f > len || tok.f < 0 || (len - tok.f) < 3 {
        return;
    }

    let lex = &src[tok.f as usize..(tok.f + 3) as usize];

    if lex == "MIN" || lex == "MAX" {
        tok.ttype = constants::TOKEN_EXTREMA;
        tok.lexeme = (&src[tok.f as usize..(tok.f + 3) as usize]).to_string();
        tok.f += 3;
    }
}

pub fn dfa_fmin(tok: &mut Token, src: &String) {
    let len: i32 = src.len().try_into().unwrap();

    if tok.f > len || tok.f < 0 || (len - tok.f) < 3 {
        return;
    }

    let lex = &src[tok.f as usize..(tok.f + 3) as usize];

    if lex == "min" {
        tok.ttype = constants::TOKEN_FMIN;
        tok.lexeme = (&src[tok.f as usize..(tok.f + 3) as usize]).to_string();
        tok.f += 3;
    }
}

pub fn dfa_fmax(tok: &mut Token, src: &String) {
    let len: i32 = src.len().try_into().unwrap();

    if tok.f > len || tok.f < 0 || (len - tok.f) < 3 {
        return;
    }

    let lex = &src[tok.f as usize..(tok.f + 3) as usize];

    if lex == "max" {
        tok.ttype = constants::TOKEN_FMAX;
        tok.lexeme = (&src[tok.f as usize..(tok.f + 3) as usize]).to_string();
        tok.f += 3;
    }
}

pub fn dfa_fcount(tok: &mut Token, src: &String) {
    let len: i32 = src.len().try_into().unwrap();

    if tok.f > len || tok.f < 0 || (len - tok.f) < 5 {
        return;
    }

    let lex = &src[tok.f as usize..(tok.f + 5) as usize];

    if lex == "count" {
        tok.ttype = constants::TOKEN_FCOUNT;
        tok.lexeme = (&src[tok.f as usize..(tok.f + 5) as usize]).to_string();
        tok.f += 5;
    }
}
