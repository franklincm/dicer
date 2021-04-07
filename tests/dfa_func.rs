extern crate dicer;
use dicer::lex::*;

#[test]
fn should_recognize_fmin() {
    let mut tok = Token::new();
    dfa_fmin(&mut tok, &String::from("min"));
    assert_eq!(tok.ttype, constants::TOKEN_FMIN);
    assert_eq!(tok.lexeme, "min");
    assert_eq!(tok.f, 3);
}

#[test]
fn should_not_recognize_fmin_partial() {
    let mut tok = Token::new();
    dfa_fmin(&mut tok, &String::from("mi"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_recognize_fmax() {
    let mut tok = Token::new();
    dfa_fmax(&mut tok, &String::from("max"));
    assert_eq!(tok.ttype, constants::TOKEN_FMAX);
    assert_eq!(tok.lexeme, "max");
    assert_eq!(tok.f, 3);
}

#[test]
fn should_not_recognize_fmax_partial() {
    let mut tok = Token::new();
    dfa_fmax(&mut tok, &String::from("ma"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_recognize_fcount() {
    let mut tok = Token::new();
    dfa_fcount(&mut tok, &String::from("count"));
    assert_eq!(tok.ttype, constants::TOKEN_FCOUNT);
    assert_eq!(tok.lexeme, "count");
    assert_eq!(tok.f, 5);
}

#[test]
fn should_not_recognize_fcount_partial() {
    let mut tok = Token::new();
    dfa_fcount(&mut tok, &String::from("coun"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}
