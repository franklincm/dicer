use dicer::*;
use lex::*;
use std::convert::TryInto;

fn main() {
    let test_str2 = String::from(" ** / +++ -   *");
    let len: i32 = test_str2.len().try_into().unwrap();
    let mut pos = 0;

    while pos != len {
        let tok = nfa(pos, &test_str2);
        println!("tok.ttype: {}", token_type_to_str(tok.ttype));
        pos = tok.f;
    }
}
