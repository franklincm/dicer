use dicer::*;
use lex::*;

fn main() {
    let test_str = String::from(" +");

    let mut tok1 = lex::Token::new();

    dfa_whitespace(&mut tok1, &test_str);
    println!("tok1.lexeme: \"{}\"", tok1.lexeme);
    println!("tok1.f: {}", tok1.f);
    println!("tok1.ttype: {}", token_type_to_str(tok1.ttype));

    let mut tok2 = lex::Token::new();
    tok2.f = tok1.f;
    dfa_catchall(&mut tok2, &test_str);
    println!("tok2.lexeme: \"{}\"", tok2.lexeme);
    println!("tok2.f: {}", tok2.f);
    println!("tok2.ttype: {}", token_type_to_str(tok2.ttype));
}
