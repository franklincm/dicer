use crate::lex::Token;
use crate::lex::*;
use dicer::token_type_to_str;

pub fn start(src: &String) -> Token {
    nfa(src, 0)
}

pub fn match_t(ttype: i32, token: Token, src: &String) -> Result<Token, i32> {
    // if EOF, return default token
    if token.ttype == ttype && ttype == constants::TOKEN_EOF {
        println!("** end of parse **");
        Ok(Token::new())
    } else if token.ttype == ttype {
        let mut tok = nfa(src, token.f);

        // if whitespace, skip
        if tok.ttype == constants::TOKEN_WS {
            tok = nfa(src, tok.f);
        }

        Ok(tok)

        // otherwise return error
    } else {
        eprintln!(
            "**SYNERR** Expecting {}, Received: {}",
            token_type_to_str(ttype),
            token_type_to_str(token.ttype)
        );
        Err(constants::TOKEN_SYNERR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_and_return_next_token() {
        let test_str = String::from("1d20");
        let mut previous_token = Token::new();
        previous_token.ttype = constants::TOKEN_NUM;
        previous_token.f = 1;
        let tok: Token = match_t(constants::TOKEN_NUM, previous_token, &test_str).unwrap();
        assert_eq!(tok.ttype, constants::TOKEN_D);
    }

    #[test]
    fn should_synerr() {
        let test_str = String::from("1d20");
        let mut previous_token = Token::new();
        previous_token.ttype = constants::TOKEN_NUM;
        previous_token.f = 1;
        assert_eq!(
            match_t(constants::TOKEN_RELOP, previous_token, &test_str).err(),
            Some(constants::TOKEN_SYNERR)
        );
    }
}
