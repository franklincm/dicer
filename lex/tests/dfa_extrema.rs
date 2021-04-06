use lex::*;

#[test]
fn should_recognize_min() {
    let mut tok = Token::new();
    dfa_extrema(&mut tok, &String::from("MIN"));
    assert_eq!(tok.ttype, constants::TOKEN_EXTREMA);
    assert_eq!(tok.lexeme, "MIN");
    assert_eq!(tok.f, 3);
}

#[test]
fn should_recognize_max() {
    let mut tok = Token::new();
    dfa_extrema(&mut tok, &String::from("MAX"));
    assert_eq!(tok.ttype, constants::TOKEN_EXTREMA);
    assert_eq!(tok.lexeme, "MAX");
    assert_eq!(tok.f, 3);
}

#[test]
fn should_not_recognize_fmin() {
    let mut tok = Token::new();
    dfa_extrema(&mut tok, &String::from("min"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}

#[test]
fn should_not_recognize_fmax() {
    let mut tok = Token::new();
    dfa_extrema(&mut tok, &String::from("max"));
    assert_eq!(tok.ttype, constants::TOKEN_UNRECSYM);
    assert_eq!(tok.lexeme, "");
    assert_eq!(tok.f, 0);
}
