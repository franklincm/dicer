use lex::*;

fn main() {
    let t = dfa_whitespace("            1d20 + 4", 0);
    println!("{}", t.lexeme);
}
