pub struct Lexer<'a> {
    input: &'a [u8],
    input_len: usize,
    current_pos: usize,
}

fn is_whitespace(c: u8) -> bool {
    // ASCII codes 9 thru 13 and 32 are all
    // whitespaces char codes
    if (9..13).contains(&c) || c == 32 {
        return true;
    }

    return false;
}

impl<'a> Lexer<'a> {
    pub fn new(f: &'a [u8]) -> Self {
        Self {
            input: f,
            input_len: f.len() - 1,
            current_pos: 0,
        }
    }

    pub fn get_token(&mut self) -> Option<u8> {
        if self.current_pos <= self.input_len {
            while (is_whitespace(self.input[self.current_pos])) {
                // If we are currently at the final index of the input (or somehow over it) and discover that the final character
                // is whitespace, break out early and return None to avoid indexing OOB
                if self.current_pos >= self.input_len {
                    return None;
                } else {
                    self.current_pos += 1;
                }
            }

            self.current_pos += 1;
            return Some(self.input[self.current_pos - 1]);
        }

        return None;
    }

    pub fn peek_token(&mut self) -> Option<u8> {
        let peek_val = self.get_token();
        self.current_pos -= 1;
        return peek_val;
    }

    pub fn rewind_input(&mut self) {
        self.current_pos = 0;
    }

    pub fn rewind_input_steps(&mut self, rewind_steps: usize) -> bool {
        let mut x = rewind_steps;

        if self.current_pos.checked_sub(rewind_steps).is_none() {
            return false;
        }

        while x != 0 {
            while is_whitespace(self.input[self.current_pos]) {
                self.current_pos -= 1;
                x -= 1;
            }
            x -= 1;
            self.current_pos -= 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: [u8; 7] = [3, 5, 2, 3, 10, 24, 32];

    #[test]
    fn test_init() {
        let test_lex = Lexer::new(&TEST_DATA);
        assert!(test_lex.input_len == 6);
        assert!(test_lex.input == TEST_DATA);
    }

    #[test]
    fn test_rewind_input() {
        let mut test_lex = Lexer::new(&TEST_DATA);
        test_lex.rewind_input();
        assert!(test_lex.current_pos == 0);
    }

    #[test]
    fn test_rewind_input_steps() {
        let mut test_lex = Lexer::new(&TEST_DATA);
        test_lex.current_pos = 6;
        assert!((test_lex.rewind_input_steps(5)) && test_lex.current_pos == 1);
        assert!(!(test_lex.rewind_input_steps(11)));
    }

    #[test]
    fn test_get_token() {
        let mut test_lex = Lexer::new(&TEST_DATA);
        assert!(test_lex.get_token() == Some(3));
        assert!(test_lex.get_token() == Some(5));
        assert!(test_lex.get_token() == Some(2));
        assert!(test_lex.get_token() == Some(3));
        assert!(test_lex.get_token() == Some(24));
        assert!(test_lex.get_token() == None);
        assert!(test_lex.get_token() == None);
    }

    #[test]
    fn test_peek_token() {
        let mut test_lex = Lexer::new(&TEST_DATA);
        assert!(test_lex.peek_token() == Some(3));
        _ = test_lex.get_token();
        assert!(test_lex.peek_token() == Some(5));
        _ = test_lex.get_token();
        assert!(test_lex.peek_token() == Some(2));
        _ = test_lex.get_token();
        assert!(test_lex.peek_token() == Some(3));
        _ = test_lex.get_token();
        assert!(test_lex.peek_token() == Some(24));
        _ = test_lex.get_token();
        assert!(test_lex.peek_token() == None);
    }
}
