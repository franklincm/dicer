use crate::lex::constants;
use crate::lex::RollResult;
use crate::lex::Token;
use crate::parse;

use rand::{thread_rng, Rng};
use std::cmp;

pub fn parse_expression(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
        || token.ttype == constants::TOKEN_LBRACKET
    {
        parse_simple_expression(token, src);
    } else if token.ttype == constants::TOKEN_FCOUNT {
        parse_fcount(token, src);
    }
    print!("\n");
    parse::match_t(constants::TOKEN_EOF, token, src);
}

pub fn parse_simple_expression(token: &mut Token, src: &String) {
    parse_term(token, src);
    parse_simple_expression_tail(token, src);
}

pub fn parse_simple_expression_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_ADDOP {
        let op = token.lexeme.clone();
        let result = token.carry;
        print!(" {} ", op);
        parse::match_t(constants::TOKEN_ADDOP, token, src);

        parse_term(token, src);

        if op == "+" {
            token.carry = result + token.carry;
        } else if op == "-" {
            token.carry = result - token.carry;
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
        let result = token.carry;

        parse::match_t(constants::TOKEN_MULOP, token, src);

        parse_factor(token, src);
        if op == "*" {
            token.carry = result * token.carry;
        } else if op == "/" {
            token.carry = result / token.carry;
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
        token.carry = token.attr;

        parse::match_t(constants::TOKEN_NUM, token, src);

        parse_factor_tail(token, src);
    } else if token.ttype == constants::TOKEN_LBRACKET {
        parse::match_t(constants::TOKEN_LBRACKET, token, src);

        let num_dice = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src);
        parse::match_t(constants::TOKEN_D, token, src);

        let num_sides = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src);

        token.result = roll(num_dice, num_sides);
        token.carry = token.result.sum;

        let op = token.lexeme.clone();
        print!("[ (");
        let mut rolls: Vec<String> = Vec::new();
        for val in &token.result.values {
            rolls.push(val.to_string());
        }
        let rolls_str = rolls.join(" + ");
        print!("{}", rolls_str);

        print!(") {}", op);

        parse::match_t(constants::TOKEN_ADDOP, token, src);
        let extrema = token.lexeme.clone();
        parse::match_t(constants::TOKEN_EXTREMA, token, src);

        if op == "+" && extrema == "MAX" {
            print!("{} ]", token.result.max);
            token.carry += token.result.max;
        } else if op == "-" && extrema == "MAX" {
            print!("{} ]", token.result.max);
            token.carry -= token.result.max;
        } else if op == "+" && extrema == "MIN" {
            print!("{} ]", token.result.min);
            token.carry += token.result.min;
        } else if op == "-" && extrema == "MIN" {
            print!("{} ]", token.result.min);
            token.carry -= token.result.min;
        }

        parse::match_t(constants::TOKEN_RBRACKET, token, src);
    } else if token.ttype == constants::TOKEN_LPAREN {
        parse::match_t(constants::TOKEN_LPAREN, token, src);
        parse_simple_expression(token, src);
        parse::match_t(constants::TOKEN_RPAREN, token, src);
    } else if token.ttype == constants::TOKEN_FMIN {
        parse_fmin(token, src);
    } else if token.ttype == constants::TOKEN_FMAX {
        parse_fmax(token, src);
    } else {
        token.ttype = constants::TOKEN_SYNERR;
    }
}

pub fn parse_factor_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_D {
        parse::match_t(constants::TOKEN_D, token, src);

        token.result = roll(token.carry, token.attr);
        token.carry = token.result.sum;

        print!("(");
        let mut rolls: Vec<String> = Vec::new();
        for val in &token.result.values {
            rolls.push(val.to_string());
        }
        let rolls_str = rolls.join(" + ");
        print!("{}", rolls_str);

        print!(")");

        parse::match_t(constants::TOKEN_NUM, token, src);
    } else {
        print!("{}", token.carry);
    }
}

pub fn parse_fmin(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMIN, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);
    print!("min(");
    parse_simple_expression(token, src);
    print!(", ");
    let first = token.carry;

    parse::match_t(constants::TOKEN_COMMA, token, src);
    parse_simple_expression(token, src);
    let second = token.carry;
    print!(")");
    token.carry = cmp::min(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

pub fn parse_fmax(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMAX, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);
    print!("max(");
    parse_simple_expression(token, src);
    print!(", ");
    let first = token.carry;

    parse::match_t(constants::TOKEN_COMMA, token, src);
    parse_simple_expression(token, src);
    let second = token.carry;
    print!(")");
    token.carry = cmp::max(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

pub fn parse_fcount(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FCOUNT, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);

    let first = token.attr;
    parse::match_t(constants::TOKEN_NUM, token, src);
    parse::match_t(constants::TOKEN_D, token, src);

    let second = token.attr;
    parse::match_t(constants::TOKEN_NUM, token, src);

    print!("count({}d{}, ", first, second);

    token.result = roll(first, second);
    token.carry = token.result.sum;

    parse::match_t(constants::TOKEN_COMMA, token, src);

    parse_condition_list(token, src);

    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

pub fn parse_condition_list(token: &mut Token, src: &String) {
    parse_condition(token, src);
    parse_condition_list_tail(token, src);
    print!(")");
}

pub fn parse_condition(token: &mut Token, src: &String) {
    let relop = token.lexeme.clone();
    parse::match_t(constants::TOKEN_RELOP, token, src);
    let val = token.attr;
    print!("{}{}: ", relop, val);
    let num;
    if relop == ">" {
        num = token.result.values.iter().filter(|&n| *n > val).count();
        print!("{}", num);
    } else if relop == "<" {
        num = token.result.values.iter().filter(|&n| *n < val).count();
        print!("{}", num);
    } else if relop == ">=" {
        num = token.result.values.iter().filter(|&n| *n >= val).count();
        print!("{}", num);
    } else if relop == "<=" {
        num = token.result.values.iter().filter(|&n| *n <= val).count();
        print!("{}", num);
    } else if relop == "=" {
        num = token.result.values.iter().filter(|&n| *n == val).count();
        print!("{}", num);
    }

    parse::match_t(constants::TOKEN_NUM, token, src);
}

pub fn parse_condition_list_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_COMMA {
        print!(", ");
        parse::match_t(constants::TOKEN_COMMA, token, src);

        parse_condition(token, src);

        parse_condition_list_tail(token, src);
    }
}

pub fn roll(n: i32, m: i32) -> RollResult {
    let mut rng = thread_rng();
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut values: Vec<i32> = Vec::new();
    let mut i;

    for _ in 1..(n + 1) {
        i = rng.gen_range(1..(m + 1));
        if i > max {
            max = i;
        }

        if i < min {
            min = i;
        }

        values.push(i)
    }

    RollResult {
        sum: values.iter().sum(),
        min: min,
        max: max,
        values: values,
    }
}
