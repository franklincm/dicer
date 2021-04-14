mod lex;
mod parse;

use lex::*;

fn main() {
    // simple expression
    //let test = String::from("1d2 * (3+2) + min(2, 3)");
    //let test = String::from("2d20 - [ 2d4+MAX ]");
    let test = String::from("[4d6-MIN] + min(4,2d4)");
    println!("parse: \"{}\"\n", test);
    let mut token: Token = parse::start(&test);
    parse::rdp::parse_expression(&mut token, &test);

    // fcount
    // let test = String::from("count(5d6, >1, <=5)");
    // println!("\nparse: \"{}\"\n", test);
    // let mut token: Token = parse::start(&test);
    // parse::rdp::parse_expression(&mut token, &test);
}
