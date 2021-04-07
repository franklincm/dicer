use dicer::*;
use lex::*;

fn main() {
    let test = String::from("1d20 + 5 * (2d12) <= 5 + MIN - MAX + min()");

    let tokens: Vec<Token> = lex(&test);

    for tok in &tokens {
        println!("{}", token_type_to_str(tok.ttype));
    }
}
