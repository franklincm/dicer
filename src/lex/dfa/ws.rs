use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn m_whitespace(tok: &mut Token, src: &String) {
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
