#[derive(Debug, PartialEq)]
pub enum Token {
    Hash(String),
    Word(String),
    Commit,
    Author,
    Date,
    NewLine,
    LParen,
    RParen,
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
            "commit" => Token::Commit,
            "Author:" => Token::Author,
            "Date:" => Token::Date,
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
    const PREFIX: &'static str = r#"commit 0b5a4e8d5a1ae5b6d5539e3fc7023e0f3faf77af (HEAD -> master, origin/master)
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 15:58:03 2023 -0500

"#;

    fn gen_prefix_matches() -> Vec<Token> {
        use Token::*;
        vec![
            Commit,
            Hash("0b5a4e8d5a1ae5b6d5539e3fc7023e0f3faf77af".into()),
            LParen,
            Word("HEAD".into()),
            Word("->".into()),
            Word("master,".into()),
            Word("origin/master".into()),
            RParen,
            NewLine,
            Author,
            Word("Matt".into()),
            Word("Christofides".into()),
            Word("<matt.christofides@gmail.com>".into()),
            NewLine,
            Date,
            Word("Sat".into()),
            Word("Nov".into()),
            Word("25".into()),
            Word("15:58:03".into()),
            Word("2023".into()),
            Word("-0500".into()),
            NewLine,
            NewLine,
        ]
    }

    #[test]
    fn testing_keywords_in_message() {
        let input = format!("{PREFIX} HEAD commit Author: Date: test");
        use Token::*;
        let mut output = gen_prefix_matches();
        output.append(&mut vec![
            Word("HEAD".into()),
            Commit,
            Author,
            Date,
            Word("test".into()),
            EOF,
        ]);

        let mut l = Lexer::new_from_string(input.into());
        for i in 0..output.len() {
            assert_eq!(output[i], l.next_token())
        }
    }

    #[test]
    fn pretty_medium_tokenize() {
        let input = format!(
            r#"{PREFIX}feat: added target dir option

commit bb4055c04da174bbfc93e63952d4ccc84e4832ab (origin/master)
Author: Matt Christofides <matt.christofides@gmail.com>"#
        );
        use Token::*;
        let mut output = gen_prefix_matches();
        output.append(&mut vec![
            Word("feat:".into()),
            Word("added".into()),
            Word("target".into()),
            Word("dir".into()),
            Word("option".into()),
            NewLine,
            NewLine,
            Commit,
            Hash("bb4055c04da174bbfc93e63952d4ccc84e4832ab".into()),
            LParen,
            Word("origin/master".into()),
            RParen,
            NewLine,
            Author,
            Word("Matt".into()),
            Word("Christofides".into()),
            Word("<matt.christofides@gmail.com>".into()),
            EOF,
        ]);

        let mut l = Lexer::new_from_string(input.into());
        for i in 0..output.len() {
            assert_eq!(output[i], l.next_token())
        }
    }
}
