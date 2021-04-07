use crate::lex::lex;
use crate::lex::Token;
use dicer::token_type_to_str;

pub fn get_tokens(src: &String) {
    let tokens: Vec<Token> = lex(src);

    for tok in &tokens {
        println!("{}", token_type_to_str(tok.ttype));
    }
}
