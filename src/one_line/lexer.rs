#[derive(Debug, PartialEq)]
pub enum Token {
    Hash(String),
    Word(String),
    Head,
    LParen,
    RParen,
    Arrow,
    NewLine,
    EOF,
    Illegal,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "HEAD" => Token::Head,
            s if s.len() == 40 && s.bytes().all(|x| x.is_ascii_hexdigit()) => Token::Hash(s.into()),
            _ => Token::Word(s.into()),
        }
    }
}

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            ch: input[0],
            input,
            pos: 0,
        }
    }

    fn read_char(&mut self) {
        if self.pos + 1 >= self.input.len() {
            self.pos += 1;
            self.ch = b'\0';
        } else {
            self.pos += 1;
            self.ch = self.input[self.pos];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() && self.ch != b'\n' {
            self.read_char()
        }
    }

    fn read_word(&mut self) -> Token {
        let s_pos = self.pos;
        while !self.ch.is_ascii_whitespace() && self.ch != b'\0' && self.ch != b')' {
            self.read_char()
        }
        let word =
            std::str::from_utf8(&self.input[s_pos..self.pos]).expect("This should be valid ascii");
        let a = word.into();
        a
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let t = match self.ch {
            b'\0' => Token::EOF,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'\n' => Token::NewLine,
            b'-' => {
                if self.input[self.pos + 1] == b'>' {
                    self.read_char();
                    Token::Arrow
                } else {
                    return self.read_word();
                }
            }
            rest if rest.is_ascii() => return self.read_word(),
            _ => Token::Illegal,
        };

        self.read_char();
        t
    }
}
#[cfg(test)]
mod tests {
    impl Lexer {
        pub fn new_from_string(s: String) -> Self {
            let input = s.into_bytes();
            Self {
                ch: input[0],
                input,
                pos: 0,
            }
        }
    }

    use super::{Lexer, Token};

    #[test]
    fn tokenize() {
        let input = r#"c03b7c35a902784aae4c3fd7fdfb8479734fda70 (HEAD -> master) Title
a42cc2e1c21d71ce016b5b878b4b1ac801a5fb83 feat: parsing done "#;
        let output = vec![
            Token::Hash("c03b7c35a902784aae4c3fd7fdfb8479734fda70".into()),
            Token::LParen,
            Token::Head,
            Token::Arrow,
            Token::Word("master".into()),
            Token::RParen,
            Token::Word("Title".into()),
            Token::NewLine,
            Token::Hash("a42cc2e1c21d71ce016b5b878b4b1ac801a5fb83".into()),
            Token::Word("feat:".into()),
            Token::Word("parsing".into()),
            Token::Word("done".into()),
            Token::EOF,
        ];

        let mut l = Lexer::new_from_string(input.into());
        for i in 0..output.len() {
            assert_eq!(output[i], l.next_token())
        }
    }
}
