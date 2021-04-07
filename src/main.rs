mod lex;
mod parse;

use parse::*;

fn main() {
    let test = String::from("1d20 + 5 * (2d12) <= 5 + MIN - MAX + min()");
    get_tokens(&test);
}
