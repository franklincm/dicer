pub mod rdp;

use crate::lex::constants;
use crate::lex::nfa;
use crate::lex::Token;
use dicer::token_type_to_str;

pub fn start(src: &String) -> Token {
    nfa(src, 0)
}

pub fn match_t<'a>(ttype: i32, token: &'a mut Token, src: &String) -> Result<&'a Token, i32> {
    let mut tok: Token;
    let result_sum = token.result.sum;
    let result_min = token.result.min;
    let result_max = token.result.max;

    let carry = token.carry;

    // if EOF, return default token
    if token.ttype == ttype && ttype == constants::TOKEN_EOF {
        println!("RESULT:::{}", token.carry);
        tok = Token::new();
        *token = tok;
        token.result.sum = result_sum;
        token.result.min = result_min;
        token.result.max = result_max;
        token.carry = carry;

        Ok(token)
    } else if token.ttype == ttype {
        // println!(
        //     "match: {} == {}",
        //     token_type_to_str(token.ttype),
        //     token_type_to_str(ttype)
        // );

        tok = nfa(src, token.f);

        // if whitespace, skip
        if tok.ttype == constants::TOKEN_WS {
            tok = nfa(src, tok.f);
        }

        *token = tok;
        token.result.sum = result_sum;
        token.result.min = result_min;
        token.result.max = result_max;
        token.carry = carry;

        Ok(token)

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
        let mut token = Token::new();
        token.ttype = constants::TOKEN_NUM;
        token.f = 1;
        match_t(constants::TOKEN_NUM, &mut token, &test_str).unwrap();
        assert_eq!(token.ttype, constants::TOKEN_D);
    }

    #[test]
    fn should_synerr() {
        let test_str = String::from("1d20");
        let mut token = Token::new();
        token.ttype = constants::TOKEN_NUM;
        token.f = 1;
        assert_eq!(
            match_t(constants::TOKEN_RELOP, &mut token, &test_str).err(),
            Some(constants::TOKEN_SYNERR)
        );
    }
}
