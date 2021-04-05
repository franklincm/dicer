use lex::Token;

fn main() {
    println!("Hello, world!");

    let t = Token {
        ttype: 1,
        lexeme: String::from("test"),
        attr: 1,
        f: 0,
    };
}
