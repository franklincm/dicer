use lex::*;

#[test]
fn should_recognize_less_than() {
    let mut tok = Token::new();
    dfa_relop(&mut tok, &String::from("<"));
    assert_eq!(tok.ttype, constants::TOKEN_RELOP);
    assert_eq!(tok.lexeme, "<");
    assert_eq!(tok.f, 1);
}

#[test]
fn should_recognize_greater_than() {
    let mut tok = Token::new();
    dfa_relop(&mut tok, &String::from(">"));
    assert_eq!(tok.ttype, constants::TOKEN_RELOP);
    assert_eq!(tok.lexeme, ">");
    assert_eq!(tok.f, 1);
}

#[test]
fn should_recognize_less_than_eq() {
    let mut tok = Token::new();
    dfa_relop(&mut tok, &String::from("<="));
    assert_eq!(tok.ttype, constants::TOKEN_RELOP);
    assert_eq!(tok.lexeme, "<=");
    assert_eq!(tok.f, 2);
}

#[test]
fn should_recognize_greater_than_eq() {
    let mut tok = Token::new();
    dfa_relop(&mut tok, &String::from(">="));
    assert_eq!(tok.ttype, constants::TOKEN_RELOP);
    assert_eq!(tok.lexeme, ">=");
    assert_eq!(tok.f, 2);
}
