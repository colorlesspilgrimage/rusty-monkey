pub struct Lexer<'a> {
    input: std::str::SplitWhitespace<'a>,
}

#[derive(PartialEq)]
pub enum Token {
    Let(String),
    IsMut(String),
    AssignOrEq(String),
    MathOp(String),
    Literal(String),
    Numerical(String),
    LineEnd(String),
    // TODO: We still need to implement function call parsing here
}

impl<'a> Lexer<'a> {
    pub fn new(f: &'a String) -> Self {
        Self {
            input: f.split_whitespace(),
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        let token = String::from(self.input.next().unwrap());

        if token.chars().all(|c| c.is_ascii_digit()) {
            return Some(Token::Numerical(token));
        } else {
            return match token.as_str() {
                "let" => Some(Token::Let(token)),
                "mut" => Some(Token::IsMut(token)),
                "=" => Some(Token::AssignOrEq(token)),
                "+" | "-" | "/" | "*" => Some(Token::MathOp(token)),
                ";" => Some(Token::LineEnd(token)),
                _ => Some(Token::Literal(token)),
            };
        }
    }
}

// TODO: all tests are now failing since we re-wrote the lexer logic to operate on enumerated tokens.
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: String = String::from(
        "
    let mut x = 22 * 22;
",
    );

    #[test]
    fn test_get_token() {
        let mut test_lex = Lexer::new(&TEST_DATA);
        assert!(Some(Token::Let(_)) == test_lex.get_token());
    }
}
