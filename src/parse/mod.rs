use crate::lex::constants;
use crate::lex::lex;
use crate::lex::Token;
use dicer::token_type_to_str;

pub fn get_tokens(src: &String) {
    let tokens: Vec<Token> = lex(src);

    for tok in &tokens {
        if tok.ttype == constants::TOKEN_NUM {
            println!("{} = {}", token_type_to_str(tok.ttype), tok.attr);
        } else {
            println!("{}", token_type_to_str(tok.ttype));
        }
    }
}
