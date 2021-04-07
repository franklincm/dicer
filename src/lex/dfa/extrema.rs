use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_recognize_min() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("MIN"));
        assert_eq!(tok.ttype, constants::TOKEN_EXTREMA);
        assert_eq!(tok.lexeme, "MIN");
        assert_eq!(tok.f, 3);
    }

    #[test]
    fn should_recognize_max() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("MAX"));
        assert_eq!(tok.ttype, constants::TOKEN_EXTREMA);
        assert_eq!(tok.lexeme, "MAX");
        assert_eq!(tok.f, 3);
    }

    #[test]
    fn should_not_recognize_fmin() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("min"));
        assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }

    #[test]
    fn should_not_recognize_fmax() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("max"));
        assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }
}
