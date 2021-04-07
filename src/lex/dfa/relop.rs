use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_recognize_less_than() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("<"));
        assert_eq!(tok.ttype, constants::TOKEN_RELOP);
        assert_eq!(tok.lexeme, "<");
        assert_eq!(tok.f, 1);
    }

    #[test]
    fn should_recognize_greater_than() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from(">"));
        assert_eq!(tok.ttype, constants::TOKEN_RELOP);
        assert_eq!(tok.lexeme, ">");
        assert_eq!(tok.f, 1);
    }

    #[test]
    fn should_recognize_less_than_eq() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("<="));
        assert_eq!(tok.ttype, constants::TOKEN_RELOP);
        assert_eq!(tok.lexeme, "<=");
        assert_eq!(tok.f, 2);
    }

    #[test]
    fn should_recognize_greater_than_eq() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from(">="));
        assert_eq!(tok.ttype, constants::TOKEN_RELOP);
        assert_eq!(tok.lexeme, ">=");
        assert_eq!(tok.f, 2);
    }
}
