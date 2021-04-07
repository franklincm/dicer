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
        Some('d') => k += 1,
        Some('D') => k += 1,
        _ => return,
    }

    if k > tok.f {
        tok.ttype = constants::TOKEN_D;
        tok.lexeme = (&src[tok.f as usize..k as usize]).to_string();
        tok.f = k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_recognize_lower_d() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("d"));
        assert_eq!(tok.ttype, constants::TOKEN_D);
        assert_eq!(tok.lexeme, "d");
        assert_eq!(tok.f, 1);
    }

    #[test]
    fn should_recognize_capital_d() {
        let mut tok = Token::new();
        machine(&mut tok, &String::from("D"));
        assert_eq!(tok.ttype, constants::TOKEN_D);
        assert_eq!(tok.lexeme, "D");
        assert_eq!(tok.f, 1);
    }
}
