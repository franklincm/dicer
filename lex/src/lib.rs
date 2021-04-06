mod constants;
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
            ttype: constants::TOKEN_LEXERR,
            lexeme: String::from(""),
            attr: 0,
            f: 0,
        }
    }
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

    tok.ttype = constants::TOKEN_WS;
    tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
    tok.f = k;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_offset() {
        let mut tok = Token::new();

        dfa_whitespace(&mut tok, &String::from(" 1d20 + 4"));
        assert_eq!(tok.ttype, constants::TOKEN_WS);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(" "));
    }

    #[test]
    fn test_ws_no_offset() {
        let mut tok = Token::new();

        dfa_whitespace(&mut tok, &String::from("1d20 + 4"));
        assert_eq!(tok.ttype, constants::TOKEN_WS);
        assert_eq!(tok.f, 0);
        assert_eq!(tok.lexeme, String::from(""));
    }

    #[test]
    fn test_ws_offset_too_large() {
        let mut tok = Token::new();
        tok.f = 20;

        dfa_whitespace(&mut tok, &String::from("1d20 + 4"));
        assert_eq!(tok.ttype, constants::TOKEN_LEXERR);
        assert_eq!(tok.f, 20);
        assert_eq!(tok.lexeme, String::from(""));
    }

    #[test]
    fn test_ws_offset_too_small() {
        let mut tok = Token::new();
        tok.f = -20;

        dfa_whitespace(&mut tok, &String::from("1d20 + 4"));
        assert_eq!(tok.ttype, constants::TOKEN_LEXERR);
        assert_eq!(tok.f, -20);
        assert_eq!(tok.lexeme, String::from(""));
    }
}
