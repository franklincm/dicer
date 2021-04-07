use crate::lex::constants;
use crate::lex::Token;
use std::convert::TryInto;

pub fn machine(tok: &mut Token, src: &String) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_recognize_fcount() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("count"));
        assert_eq!(tok.ttype, constants::TOKEN_FCOUNT);
        assert_eq!(tok.lexeme, "count");
        assert_eq!(tok.f, 5);
    }

    #[test]
    fn should_not_recognize_fcount_partial() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("coun"));
        assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
        assert_eq!(tok.lexeme, "");
        assert_eq!(tok.f, 0);
    }
}
