use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_recognize_int() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("100"));
        assert_eq!(tok.ttype, constants::TOKEN_NUM);
        assert_eq!(tok.lexeme, "100");
        assert_eq!(tok.f, 3);
    }

    #[test]
    fn should_not_recognize_leading_zero() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("0100"));
        assert_eq!(tok.ttype, constants::TOKEN_LEXERR);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }

    #[test]
    fn should_stop_reading_at_non_int_characters() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("100d100"));
        assert_eq!(tok.ttype, constants::TOKEN_NUM);
        assert_eq!(tok.lexeme, "100");
        assert_eq!(tok.f, 3);
    }
}
