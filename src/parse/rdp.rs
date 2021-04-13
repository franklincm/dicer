use crate::lex::constants;
use crate::lex::Token;
use crate::parse;

use rand::{thread_rng, Rng};

pub fn parse_expression(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
    {
        parse_simple_expression(token, src);
    } else if token.ttype == constants::TOKEN_FCOUNT {
        parse_fcount(token, src);
    }
    parse::match_t(constants::TOKEN_EOF, token, src).unwrap();
}

pub fn parse_simple_expression(token: &mut Token, src: &String) {
    parse_term(token, src);

    parse_simple_expression_tail(token, src);
}

pub fn parse_simple_expression_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_ADDOP {
        let op = token.lexeme.clone();
        let result = token.result.0;

        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();

        parse_term(token, src);

        if op == "+" {
            token.result.0 = result + token.result.0;
        } else if op == "-" {
            token.result.0 = result - token.result.0;
        }

        parse_simple_expression_tail(token, src);
    }
}

pub fn parse_term(token: &mut Token, src: &String) {
    parse_factor(token, src);
    parse_term_tail(token, src);
}

pub fn parse_term_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_MULOP {
        let op = token.lexeme.clone();
        let result = token.result.0;

        parse::match_t(constants::TOKEN_MULOP, token, src).unwrap();

        parse_factor(token, src);

        if op == "*" {
            token.result.0 = result * token.result.0;
        } else if op == "/" {
            token.result.0 = result / token.result.0;
        }

        parse_term_tail(token, src);
    } else if token.ttype == constants::TOKEN_ADDOP
        || token.ttype == constants::TOKEN_COMMA
        || token.ttype == constants::TOKEN_RPAREN
        || token.ttype == constants::TOKEN_EOF
    {
    }
}

pub fn parse_factor(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM {
        token.result.0 = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();

        parse_factor_tail(token, src);
    } else if token.ttype == constants::TOKEN_LBRACKET {
        parse::match_t(constants::TOKEN_LBRACKET, token, src).unwrap();

        let num_dice = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
        parse::match_t(constants::TOKEN_D, token, src).unwrap();

        let num_sides = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();

        token.result = roll(num_dice, num_sides);

        let op = token.lexeme.clone();

        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();
        let extrema = token.lexeme.clone();
        parse::match_t(constants::TOKEN_EXTREMA, token, src).unwrap();

        if op == "+" && extrema == "MAX" {
            token.result.0 += token.result.1;
        } else if op == "-" && extrema == "MAX" {
            token.result.0 -= token.result.1;
        } else if op == "+" && extrema == "MIN" {
            token.result.0 += token.result.2;
        } else if op == "-" && extrema == "MIN" {
            token.result.0 -= token.result.2;
        }

        parse::match_t(constants::TOKEN_RBRACKET, token, src).unwrap();
    } else if token.ttype == constants::TOKEN_LPAREN {
        parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
        parse_simple_expression(token, src);
        parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
    } else if token.ttype == constants::TOKEN_FMIN {
        parse_fmin(token, src);
    } else if token.ttype == constants::TOKEN_FMAX {
        parse_fmax(token, src);
    } else {
        panic!("SYNERR");
    }
}

pub fn parse_factor_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_D {
        parse::match_t(constants::TOKEN_D, token, src).unwrap();

        token.result = roll(token.result.0, token.attr);

        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    }
}

pub fn parse_fmin(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMIN, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();

    parse_simple_expression(token, src);

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    parse_simple_expression(token, src);

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fmax(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMAX, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();

    parse_simple_expression(token, src);

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    parse_simple_expression(token, src);

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fcount(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FCOUNT, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_D, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    parse_condition_list(token, src);

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_condition_list(token: &mut Token, src: &String) {
    parse_condition(token, src);

    parse_condition_list_tail(token, src);
}

pub fn parse_condition(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_RELOP, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
}

pub fn parse_condition_list_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_COMMA {
        parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

        parse_condition(token, src);

        parse_condition_list_tail(token, src);
    }
}

pub fn roll(n: i32, m: i32) -> (i32, i32, i32) {
    let mut rng = thread_rng();
    let mut sum = 0;
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut intermediate;

    for _ in 1..(n + 1) {
        intermediate = rng.gen_range(1..(m + 1));
        if intermediate > max {
            max = intermediate;
        }

        if intermediate < min {
            min = intermediate;
        }

        sum += intermediate;
    }
    (sum, max, min)
}
