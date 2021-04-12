use crate::lex::Token;
use crate::lex::*;
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
        parse_term(token, src);
        print_return("expression");
    }
}

pub fn parse_term(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_NUM
        || token.ttype == constants::TOKEN_LPAREN
        || token.ttype == constants::TOKEN_FMIN
        || token.ttype == constants::TOKEN_FMAX
    {
        parse::match_t(constants::TOKEN_NUM, token, src).unwrap();
        print_descent("term", "term_tail");
        parse_term_tail(token, src);
        print_return("term");
    }
}

pub fn parse_term_tail(token: &mut Token, src: &String) {
    if token.ttype == constants::TOKEN_MULOP {
        parse::match_t(constants::TOKEN_MULOP, token, src).unwrap();
        // parse_factor
        parse_term_tail(token, src);
    } else if token.ttype == constants::TOKEN_ADDOP
        || token.ttype == constants::TOKEN_COMMA
        || token.ttype == constants::TOKEN_RPAREN
        || token.ttype == constants::TOKEN_EOF
    {
        // epsilon
    }
}
