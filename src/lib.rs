pub mod lex;
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
        7 => String::from("TOKEN_D"),
        8 => String::from("TOKEN_NUM"),
        9 => String::from("TOKEN_RELOP"),
        10 => String::from("TOKEN_EXTREMA"),
        11 => String::from("TOKEN_FMIN"),
        12 => String::from("TOKEN_FMAX"),
        13 => String::from("TOKEN_FCOUNT"),
        _ => String::from("TOKEN_UNRECSYM"),
    }
}
