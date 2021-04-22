use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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
    } else if char == &'[' {
        k += 1;
        tok.ttype = constants::TOKEN_LBRACKET;
    } else if char == &']' {
        k += 1;
        tok.ttype = constants::TOKEN_RBRACKET;
    } else if char == &'{' {
        k += 1;
        tok.ttype = constants::TOKEN_LCBRACKET;
    } else if char == &'}' {
        k += 1;
        tok.ttype = constants::TOKEN_RCBRACKET;
    }

    if k > tok.f {
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_catchall_star() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("*"));
        assert_eq!(tok.ttype, constants::TOKEN_MULOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("*"));
    }

    #[test]
    fn test_catchall_div() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("/"));
        assert_eq!(tok.ttype, constants::TOKEN_MULOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("/"));
    }

    #[test]
    fn test_catchall_plus() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("+"));
        assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("+"));
    }

    #[test]
    fn test_catchall_dash() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("-"));
        assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("-"));
    }

    #[test]
    fn test_catchall_comma() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from(","));
        assert_eq!(tok.ttype, constants::TOKEN_COMMA);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(","));
    }

    #[test]
    fn test_catchall_lparen() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("("));
        assert_eq!(tok.ttype, constants::TOKEN_LPAREN);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from("("));
    }

    #[test]
    fn test_catchall_rparen() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from(")"));
        assert_eq!(tok.ttype, constants::TOKEN_RPAREN);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(")"));
    }

    #[test]
    fn test_catchall_eof() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from(""));
        assert_eq!(tok.ttype, constants::TOKEN_EOF);
        assert_eq!(tok.f, 1);
        assert_eq!(tok.lexeme, String::from(""));
    }
}
