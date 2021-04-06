pub fn token_type_to_str(ttype: i32) -> String {
    match ttype {
        99 => String::from("TOKEN_LEXERR"),
        0 => String::from("TOKEN_WS"),
        1 => String::from("TOKEN_ADDOP"),
        2 => String::from("TOKEN_MULOP"),
        3 => String::from("TOKEN_COMMA"),
        4 => String::from("TOKEN_LPAREN"),
        5 => String::from("TOKEN_RPAREN"),
        6 => String::from("TOKEN_EOF"),
        _ => String::from("TOKEN_UNREC"),
    }
}
