use lex::*;

#[test]
fn should_recognize_lower_d() {
    let mut tok = Token::new();
    dfa_d(&mut tok, &String::from("d"));
    assert_eq!(tok.ttype, constants::TOKEN_D);
    assert_eq!(tok.lexeme, "d");
    assert_eq!(tok.f, 1);
}

#[test]
fn should_recognize_capital_d() {
    let mut tok = Token::new();
    dfa_d(&mut tok, &String::from("D"));
    assert_eq!(tok.ttype, constants::TOKEN_D);
    assert_eq!(tok.lexeme, "D");
    assert_eq!(tok.f, 1);
}
