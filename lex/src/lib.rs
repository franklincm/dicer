use std::convert::TryInto;

pub struct Token {
    pub ttype: i32,
    pub lexeme: String,
    pub attr: i32,
    pub f: i32,
}

pub fn dfa_whitespace(src: &str, i: i32) -> i32 {
    let mut k = i;
    let len: i32 = src.len().try_into().unwrap();

    if k > len || k < 0 {
        return -1;
    }

    while k < len && &src.chars().nth(k.try_into().unwrap()).unwrap() == &' ' {
        k += 1;
    }

    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_offset() {
        assert_eq!(dfa_whitespace(" hi", 0), 1);
    }

    #[test]
    fn test_ws_no_offset() {
        assert_eq!(dfa_whitespace("hi", 0), 0);
    }

    #[test]
    fn test_ws_offset_too_large() {
        assert_eq!(dfa_whitespace("hi", 3), -1);
    }

    #[test]
    fn test_ws_negative_offset() {
        assert_eq!(dfa_whitespace("hi", -1), -1);
    }
}
