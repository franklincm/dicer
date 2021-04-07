extern crate dicer;
use dicer::lex::*;

#[test]
fn test_ws_offset() {
    let mut tok = Token::new();

    dfa::ws::m_whitespace(&mut tok, &String::from(" 1d20 + 4"));
    assert_eq!(tok.ttype, constants::TOKEN_WS);
    assert_eq!(tok.f, 1);
    assert_eq!(tok.lexeme, String::from(" "));
}

#[test]
fn test_ws_no_offset() {
    let mut tok = Token::new();

    dfa::ws::m_whitespace(&mut tok, &String::from("1d20 + 4"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.f, 0);
    assert_eq!(tok.lexeme, String::from(""));
}

#[test]
fn test_ws_offset_too_large() {
    let mut tok = Token::new();
    tok.f = 20;

    dfa::ws::m_whitespace(&mut tok, &String::from("1d20 + 4"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.f, 20);
    assert_eq!(tok.lexeme, String::from(""));
}

#[test]
fn test_ws_offset_too_small() {
    let mut tok = Token::new();
    tok.f = -20;

    dfa::ws::m_whitespace(&mut tok, &String::from("1d20 + 4"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.f, -20);
    assert_eq!(tok.lexeme, String::from(""));
}
