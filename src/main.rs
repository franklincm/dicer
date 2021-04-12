mod lex;
mod parse;

use dicer::*;
use lex::*;
use parse::*;

fn main() {
    let test = String::from("1d20 + 5 * (2d12) <= 5 + MIN - MAX + min()");
    let mut token: Token = start(&test);

    println!("{}", token_type_to_str(token.ttype));

    match_t(token.ttype, &mut token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    match_t(constants::TOKEN_D, &mut token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    match_t(constants::TOKEN_NUM, &mut token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    match_t(constants::TOKEN_ADDOP, &mut token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));

    match_t(constants::TOKEN_NUM, &mut token, &test).unwrap();
    println!("{}", token_type_to_str(token.ttype));
}
