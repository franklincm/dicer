use lex::*;

fn main() {
    let mut tok = lex::Token::new();
    dfa_whitespace(&mut tok, &String::from("   1d20 + 4"));
    
    println!("tok.lexeme: \"{}\"", tok.lexeme);
    println!("tok.f: {}", tok.f);
}
