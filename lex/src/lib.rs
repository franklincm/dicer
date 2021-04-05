#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub f: i32,
}
