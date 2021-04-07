extern crate dicer;
use dicer::lex::*;

#[test]
fn should_recognize_int() {
    let mut tok = Token::new();
    dfa_num(&mut tok, &String::from("100"));
    assert_eq!(tok.ttype, constants::TOKEN_NUM);
    assert_eq!(tok.lexeme, "100");
    assert_eq!(tok.f, 3);
}

#[test]
fn should_not_recognize_leading_zero() {
    let mut tok = Token::new();
    dfa_num(&mut tok, &String::from("0100"));
    assert_eq!(tok.ttype, constants::TOKEN_LEXERR);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_stop_reading_at_non_int_characters() {
    let mut tok = Token::new();
    dfa_num(&mut tok, &String::from("100d100"));
    assert_eq!(tok.ttype, constants::TOKEN_NUM);
    assert_eq!(tok.lexeme, "100");
    assert_eq!(tok.f, 3);
}
