mod lex;
mod parse;

use dicer::*;
use lex::*;
use parse::*;

fn main() {
    let test = String::from("1d20 + 5 * (2d12) <= 5 + MIN - MAX + min()");

    let t: Token = start(&test);
    println!("{}", token_type_to_str(t.ttype));

    let token = match_t(t.ttype, t, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    let token = match_t(constants::TOKEN_D, token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    let token = match_t(constants::TOKEN_NUM, token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    let token = match_t(constants::TOKEN_ADDOP, token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    let token = match_t(constants::TOKEN_NUM, token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));
}
