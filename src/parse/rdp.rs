use crate::lex::constants;
use crate::lex::Token;
use crate::parse;

fn print_descent(caller: &str, callee: &str) {
    println!("{} -> {}", caller, callee);
}

fn print_return(caller: &str) {
    println!("return: {}", caller);
}

pub fn parse_expression(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
    {
        print_descent("expression", "term");
        parse_simple_expression(token, src);
        print_return("expression");
    } else if token.ttype == constants::TOKEN_FCOUNT {
        parse_fcount(token, src);
    }
    parse::match_t(constants::TOKEN_EOF, token, src).unwrap();
}

pub fn parse_simple_expression(token: &mut Token, src: &String) {
    print_descent("expression", "term");
    parse_term(token, src);
    print_return("expression");

    print_descent("expression", "expression_tail");
    parse_simple_expression_tail(token, src);
    print_return("expression");
}

pub fn parse_simple_expression_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_ADDOP {
        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();

        print_descent("expression", "term");
        parse_term(token, src);
        print_return("expression");

        print_descent("expression", "expression_tail");
        parse_simple_expression_tail(token, src);
        print_return("expression");
    }
}

pub fn parse_term(token: &mut Token, src: &String) {
    parse_factor(token, src);
    print_descent("term", "term_tail");
    parse_term_tail(token, src);
    print_return("term");
}

pub fn parse_term_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_MULOP {
        parse::match_t(constants::TOKEN_MULOP, token, src).unwrap();
        print_descent("term_tail", "factor");
        parse_factor(token, src);
        print_return("term_tail");

        print_descent("term_tail", "term_tail");
        parse_term_tail(token, src);
        print_return("term_tail");
    } else if token.ttype == constants::TOKEN_ADDOP
        || token.ttype == constants::TOKEN_COMMA
        || token.ttype == constants::TOKEN_RPAREN
        || token.ttype == constants::TOKEN_EOF
    {
    }
}

pub fn parse_factor(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM {
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
        print_descent("factor", "factor_tail");
        parse_factor_tail(token, src);
        print_return("factor");
    } else if token.ttype == constants::TOKEN_LPAREN {
        parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();

        print_descent("factor", "expression");
        parse_simple_expression(token, src);
        print_return("factor");

        parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
    } else if token.ttype == constants::TOKEN_FMIN {
        print_descent("factor", "fmin");
        parse_fmin(token, src);
        print_return("factor");
    } else if token.ttype == constants::TOKEN_FMAX {
        print_descent("factor", "fmin");
        parse_fmax(token, src);
        print_return("factor");
    } else {
        panic!("SYNERR");
    }
}

pub fn parse_factor_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_D {
        parse::match_t(constants::TOKEN_D, token, src).unwrap();
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
        print_descent("factor_tail", "factor_tail_tail");
        parse_factor_tail_tail(token, src);
        print_return("factor_tail");
    }
}

pub fn parse_factor_tail_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_ADDOP {
        parse::match_t(constants::TOKEN_ADDOP, token, src).unwrap();
        parse::match_t(constants::TOKEN_EXTREMA, token, src).unwrap();
    }
}

pub fn parse_fmin(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMIN, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();

    print_descent("fmin", "expression");
    parse_simple_expression(token, src);
    print_return("fmin");

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    print_descent("fmin", "expression");
    parse_simple_expression(token, src);
    print_return("fmin");

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fmax(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FMAX, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();

    print_descent("fmax", "expression");
    parse_simple_expression(token, src);
    print_return("fmax");

    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    print_descent("fmax", "expression");
    parse_simple_expression(token, src);
    print_return("fmax");

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_fcount(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_FCOUNT, token, src).unwrap();
    parse::match_t(constants::TOKEN_LPAREN, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_D, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
    parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

    print_descent("fcount", "condition_list");
    parse_condition_list(token, src);
    print_return("fcount");

    parse::match_t(constants::TOKEN_RPAREN, token, src).unwrap();
}

pub fn parse_condition_list(token: &mut Token, src: &String) {
    print_descent("condition_list", "condition");
    parse_condition(token, src);
    print_return("condition_list");

    print_descent("condition_list", "condition_list_tail");
    parse_condition_list_tail(token, src);
    print_return("condition_list");
}

pub fn parse_condition(token: &mut Token, src: &String) {
    parse::match_t(constants::TOKEN_RELOP, token, src).unwrap();
    parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
}

pub fn parse_condition_list_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_COMMA {
        parse::match_t(constants::TOKEN_COMMA, token, src).unwrap();

        print_descent("condition_list_tail", "condition");
        parse_condition(token, src);
        print_return("condition_list_tail");

        print_descent("condition_list_tail", "condition");
        parse_condition_list_tail(token, src);
        print_return("condition_list_tail");
    }
}
