pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub f: i32,
}

pub fn dfa_whitespace(src: &str, i: usize) -> Token{

    let mut k = i;
    let len = src.len();

    while k < len && &src.chars().nth(k).unwrap() == &' ' {
        k += 1;
    }

    println!("k: {}", k);
    println!("{}", &src[k..len]);

    let t = Token {
        ttype: -1,
        lexeme: src.to_string(),
        attr: 1,
        f: 0,
    };
    t
}
