mod lex;
mod parse;

//use dicer::*;
use lex::*;

fn main() {
    //let test = String::from("1d20 + 5 * (2d12) <= 5 + MIN - MAX + min()");
    let test = String::from("1 * 3");
    let mut token: Token = parse::start(&test);

    parse::rdp::parse_expression(&mut token, &test);

    // println!("{}", token_type_to_str(token.ttype));

    // parse::match_t(token.ttype, &mut token, &test).unwrap();
    // println!("{}", token_type_to_str(token.ttype));

    // parse::match_t(constants::TOKEN_D, &mut token, &test).unwrap();
    // println!("{}", token_type_to_str(token.ttype));

    // parse::match_t(constants::TOKEN_NUM, &mut token, &test).unwrap();
    // println!("{}", token_type_to_str(token.ttype));

    // parse::match_t(constants::TOKEN_ADDOP, &mut token, &test).unwrap();
    // println!("{}", token_type_to_str(token.ttype));

    // parse::match_t(constants::TOKEN_NUM, &mut token, &test).unwrap();
    // println!("{}", token_type_to_str(token.ttype));
}
