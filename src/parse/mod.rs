pub mod rdp;

use crate::lex::constants;
use crate::lex::nfa;
use crate::lex::Token;

pub fn match_t(ttype: i32, token: &mut Token, src: &str) {
    let mut tok: Token;
    let repeat = token.repeat;
    let result_sum = token.result.sum;
    let result_min = token.result.min;
    let result_max = token.result.max;
    let mut result_values: Vec<i32> = Vec::new();
    for val in &token.result.values {
        result_values.push(*val);
    }

    let carry = token.carry;

    // if EOF, return default token
    if token.ttype == ttype && ttype == constants::TOKEN_EOF {
        // nop
    } else if token.ttype == ttype {
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
        token.repeat = repeat;
        for val in result_values {
            token.result.values.push(val);
        }

        // otherwise return error
    } else {
        token.ttype = constants::TOKEN_SYNERR;
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
        match_t(constants::TOKEN_NUM, &mut token, &test_str);
        assert_eq!(token.ttype, constants::TOKEN_D);
    }

    #[test]
    fn should_synerr() {
        let test_str = String::from("1d20");
        let mut token = Token::new();
        token.ttype = constants::TOKEN_NUM;
        token.f = 1;
        match_t(constants::TOKEN_RELOP, &mut token, &test_str);
        assert_eq!(token.ttype, constants::TOKEN_SYNERR);
    }
}
