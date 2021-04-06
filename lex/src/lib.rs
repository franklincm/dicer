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

pub fn dfa_catchall(tok: &mut Token, src: &String) {
    let mut k = tok.f;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return;
    } else if k == len {
        tok.ttype = constants::TOKEN_EOF;
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

    if tok.f < k {
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
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

    #[test]
    fn test_catchall_star() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from("*"));
        assert_eq!(tok.ttype, constants::TOKEN_MULOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("*"));
    }

    #[test]
    fn test_catchall_div() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from("/"));
        assert_eq!(tok.ttype, constants::TOKEN_MULOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("/"));
    }

    #[test]
    fn test_catchall_plus() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from("+"));
        assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("+"));
    }

    #[test]
    fn test_catchall_dash() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from("-"));
        assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("-"));
    }

    #[test]
    fn test_catchall_comma() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from(","));
        assert_eq!(tok.ttype, constants::TOKEN_COMMA);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(","));
    }

    #[test]
    fn test_catchall_lparen() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from("("));
        assert_eq!(tok.ttype, constants::TOKEN_LPAREN);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("("));
    }

    #[test]
    fn test_catchall_rparen() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from(")"));
        assert_eq!(tok.ttype, constants::TOKEN_RPAREN);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(")"));
    }

    #[test]
    fn test_catchall_eof() {
        let mut tok = Token::new();
        dfa_catchall(&mut tok, &String::from(""));
        assert_eq!(tok.ttype, constants::TOKEN_EOF);
        assert_eq!(tok.f, 0);
        assert_eq!(tok.lexeme, String::from(""));
    }
}
