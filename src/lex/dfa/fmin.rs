use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_recognize_fmin() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("min"));
        assert_eq!(tok.ttype, constants::TOKEN_FMIN);
        assert_eq!(tok.lexeme, "min");
        assert_eq!(tok.f, 3);
    }

    #[test]
    fn should_not_recognize_fmin_partial() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("mi"));
        assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }
}
