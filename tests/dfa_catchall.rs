extern crate dicer;
use dicer::lex::*;

#[test]
fn test_catchall_star() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from("*"));
    assert_eq!(tok.ttype, constants::TOKEN_MULOP);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from("*"));
}

#[test]
fn test_catchall_div() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from("/"));
    assert_eq!(tok.ttype, constants::TOKEN_MULOP);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from("/"));
}

#[test]
fn test_catchall_plus() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from("+"));
    assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from("+"));
}

#[test]
fn test_catchall_dash() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from("-"));
    assert_eq!(tok.ttype, constants::TOKEN_ADDOP);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from("-"));
}

#[test]
fn test_catchall_comma() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from(","));
    assert_eq!(tok.ttype, constants::TOKEN_COMMA);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from(","));
}

#[test]
fn test_catchall_lparen() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from("("));
    assert_eq!(tok.ttype, constants::TOKEN_LPAREN);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from("("));
}

#[test]
fn test_catchall_rparen() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from(")"));
    assert_eq!(tok.ttype, constants::TOKEN_RPAREN);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from(")"));
}

#[test]
fn test_catchall_eof() {
    let mut tok = Token::new();
    dfa_catchall(&mut tok, &String::from(""));
    assert_eq!(tok.ttype, constants::TOKEN_EOF);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from(""));
}
