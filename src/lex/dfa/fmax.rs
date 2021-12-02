use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &str) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn should_recognize_fmax() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("max"));
        assert_eq!(tok.ttype, constants::TOKEN_FMAX);
        assert_eq!(tok.lexeme, "max");
        assert_eq!(tok.f, 3);
    }

    #[test]
    fn should_not_recognize_fmax_partial() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("ma"));
        assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }
}
