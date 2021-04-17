use crate::lex::constants;
use crate::lex::RollResult;
use crate::lex::Token;
use crate::parse;

use rand::{thread_rng, Rng};
use std::cmp;

pub fn parse_expression(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
        || token.ttype == constants::TOKEN_LBRACKET
    {
        parse_simple_expression(token, src, output);
        parse_repeat(token, src, output);
    } else if token.ttype == constants::TOKEN_FCOUNT {
        parse_fcount(token, src, output);
    }
    parse::match_t(constants::TOKEN_EOF, token, src);
}

fn parse_repeat(token: &mut Token, src: &String, _output: &mut String) {
    if token.ttype == constants::TOKEN_LCBRACKET {
        parse::match_t(constants::TOKEN_LCBRACKET, token, src);

        let repeats = token.attr;

        if repeats > 0 {
            // set something here to pass repeats back to caller
        }
        parse::match_t(constants::TOKEN_NUM, token, src);
        parse::match_t(constants::TOKEN_RCBRACKET, token, src);
    }
}

fn parse_simple_expression(token: &mut Token, src: &String, output: &mut String) {
    parse_term(token, src, output);
    parse_simple_expression_tail(token, src, output);
}

fn parse_simple_expression_tail(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_ADDOP {
        let op = token.lexeme.clone();
        let result = token.carry;
        output.push_str(format!(" {} ", op).as_str());
        parse::match_t(constants::TOKEN_ADDOP, token, src);

        parse_term(token, src, output);

        if op == "+" {
            token.carry = result + token.carry;
        } else if op == "-" {
            token.carry = result - token.carry;
        }

        parse_simple_expression_tail(token, src, output);
    }
}

fn parse_term(token: &mut Token, src: &String, output: &mut String) {
    parse_factor(token, src, output);
    parse_term_tail(token, src, output);
}

fn parse_term_tail(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_MULOP {
        let op = token.lexeme.clone();
        let result = token.carry;
        output.push_str(format!(" {} ", op).as_str());
        parse::match_t(constants::TOKEN_MULOP, token, src);
        parse_factor(token, src, output);

        if op == "*" {
            token.carry = result * token.carry;
        } else if op == "/" {
            token.carry = result / token.carry;
        }

        parse_term_tail(token, src, output);
    } else if token.ttype == constants::TOKEN_ADDOP
        || token.ttype == constants::TOKEN_COMMA
        || token.ttype == constants::TOKEN_RPAREN
        || token.ttype == constants::TOKEN_EOF
    {
    }
}

fn parse_factor(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_NUM {
        token.carry = token.attr;
        parse::match_t(constants::TOKEN_NUM, token, src);
        parse_factor_tail(token, src, output);
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
        output.push_str("[ (");
        let mut rolls: Vec<String> = Vec::new();

        for val in &token.result.values {
            rolls.push(val.to_string());
        }

        let rolls_str = rolls.join(" + ");
        output.push_str(format!("{}", rolls_str).as_str());
        output.push_str(format!(") {}", op).as_str());
        parse::match_t(constants::TOKEN_ADDOP, token, src);
        let extrema = token.lexeme.clone();
        parse::match_t(constants::TOKEN_EXTREMA, token, src);

        if op == "+" && extrema == "MAX" {
            output.push_str(format!("{} ]", token.result.max).as_str());
            token.carry += token.result.max;
        } else if op == "-" && extrema == "MAX" {
            output.push_str(format!("{} ]", token.result.max).as_str());
            token.carry -= token.result.max;
        } else if op == "+" && extrema == "MIN" {
            output.push_str(format!("{} ]", token.result.min).as_str());
            token.carry += token.result.min;
        } else if op == "-" && extrema == "MIN" {
            output.push_str(format!("{} ]", token.result.min).as_str());
            token.carry -= token.result.min;
        }

        parse::match_t(constants::TOKEN_RBRACKET, token, src);
    } else if token.ttype == constants::TOKEN_LPAREN {
        output.push_str("(");
        parse::match_t(constants::TOKEN_LPAREN, token, src);
        parse_simple_expression(token, src, output);
        output.push_str(")");
        parse::match_t(constants::TOKEN_RPAREN, token, src);
    } else if token.ttype == constants::TOKEN_FMIN {
        parse_fmin(token, src, output);
    } else if token.ttype == constants::TOKEN_FMAX {
        parse_fmax(token, src, output);
    } else {
        token.ttype = constants::TOKEN_SYNERR;
    }
}

fn parse_factor_tail(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_D {
        parse::match_t(constants::TOKEN_D, token, src);
        token.result = roll(token.carry, token.attr);
        token.carry = token.result.sum;
        output.push_str("(");
        let mut rolls: Vec<String> = Vec::new();

        for val in &token.result.values {
            rolls.push(val.to_string());
        }

        let rolls_str = rolls.join(" + ");
        output.push_str(format!("{}", rolls_str).as_str());
        output.push_str(")");
        parse::match_t(constants::TOKEN_NUM, token, src);
    } else {
        output.push_str(format!("{}", token.carry).as_str());
    }
}

fn parse_fmin(token: &mut Token, src: &String, output: &mut String) {
    parse::match_t(constants::TOKEN_FMIN, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);
    output.push_str("min(");
    parse_simple_expression(token, src, output);
    output.push_str(", ");
    let first = token.carry;
    parse::match_t(constants::TOKEN_COMMA, token, src);
    parse_simple_expression(token, src, output);
    let second = token.carry;
    output.push_str(")");
    token.carry = cmp::min(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

fn parse_fmax(token: &mut Token, src: &String, output: &mut String) {
    parse::match_t(constants::TOKEN_FMAX, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);
    output.push_str("max(");
    parse_simple_expression(token, src, output);
    output.push_str(", ");
    let first = token.carry;
    parse::match_t(constants::TOKEN_COMMA, token, src);
    parse_simple_expression(token, src, output);
    let second = token.carry;
    output.push_str(")");
    token.carry = cmp::max(first, second);
    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

fn parse_fcount(token: &mut Token, src: &String, output: &mut String) {
    parse::match_t(constants::TOKEN_FCOUNT, token, src);
    parse::match_t(constants::TOKEN_LPAREN, token, src);
    let first = token.attr;
    parse::match_t(constants::TOKEN_NUM, token, src);
    parse::match_t(constants::TOKEN_D, token, src);
    let second = token.attr;
    parse::match_t(constants::TOKEN_NUM, token, src);
    output.push_str(format!("count({}d{}, ", first, second).as_str());
    token.result = roll(first, second);
    token.carry = token.result.sum;
    parse::match_t(constants::TOKEN_COMMA, token, src);
    parse_condition_list(token, src, output);
    parse::match_t(constants::TOKEN_RPAREN, token, src);
}

fn parse_condition_list(token: &mut Token, src: &String, output: &mut String) {
    parse_condition(token, src, output);
    parse_condition_list_tail(token, src, output);
    output.push_str(")");
}

fn parse_condition(token: &mut Token, src: &String, output: &mut String) {
    let relop = token.lexeme.clone();
    parse::match_t(constants::TOKEN_RELOP, token, src);
    let val = token.attr;
    output.push_str(format!("{}{}: ", relop, val).as_str());
    let num;

    if relop == ">" {
        num = token.result.values.iter().filter(|&n| *n > val).count();
        output.push_str(format!("{}", num).as_str());
    } else if relop == "<" {
        num = token.result.values.iter().filter(|&n| *n < val).count();
        output.push_str(format!("{}", num).as_str());
    } else if relop == ">=" {
        num = token.result.values.iter().filter(|&n| *n >= val).count();
        output.push_str(format!("{}", num).as_str());
    } else if relop == "<=" {
        num = token.result.values.iter().filter(|&n| *n <= val).count();
        output.push_str(format!("{}", num).as_str());
    } else if relop == "=" {
        num = token.result.values.iter().filter(|&n| *n == val).count();
        output.push_str(format!("{}", num).as_str());
    }

    parse::match_t(constants::TOKEN_NUM, token, src);
}

fn parse_condition_list_tail(token: &mut Token, src: &String, output: &mut String) {
    if token.ttype == constants::TOKEN_COMMA {
        output.push_str(", ");
        parse::match_t(constants::TOKEN_COMMA, token, src);
        parse_condition(token, src, output);
        parse_condition_list_tail(token, src, output);
    }
}

fn roll(n: i32, m: i32) -> RollResult {
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
