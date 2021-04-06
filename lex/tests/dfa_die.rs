use lex::*;

#[test]
fn simple_die() {
    let mut tok = Token::new();
    dfa_die(&mut tok, &String::from("1d20"));
    assert_eq!(tok.ttype, constants::TOKEN_DIE);
    assert_eq!(tok.lexeme, "1d20");
    assert_eq!(tok.f, 4);
}

#[test]
fn should_not_recognize_single_num() {
    let mut tok = Token::new();
    dfa_die(&mut tok, &String::from("123"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_not_recognize_missing_tail() {
    let mut tok = Token::new();
    dfa_die(&mut tok, &String::from("1d"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_stop_when_not_int_char() {
    let mut tok = Token::new();
    dfa_die(&mut tok, &String::from("1d20d100"));
    assert_eq!(tok.ttype, constants::TOKEN_DIE);
    assert_eq!(tok.lexeme, "1d20");
    assert_eq!(tok.f, 4);
}

#[test]
fn should_not_recognize_zero_dice() {
    let mut tok = Token::new();
    dfa_die(&mut tok, &String::from("0d20"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
}
