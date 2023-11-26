use anyhow::{Error, Result};

use super::lexer::{Lexer, Token};
pub struct Program(pub Vec<Commit>, pub Vec<Error>);

pub struct Commit {
    pub hash: String,
    pub date: String,
    #[allow(dead_code)]
    message: String,
    #[allow(dead_code)]
    head: Option<Vec<String>>,
}

pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let curr_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            l,
            curr_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = std::mem::take(&mut self.peek_token);
        self.peek_token = self.l.next_token();
    }

    fn expect_token(&mut self, t: Token) {
        if self.curr_token != t {
            panic!("{:?},{:?}", t, self.curr_token);
            // return Err(anyhow::anyhow!("token does not match: {t:?}"));
        }
        self.next_token();
    }

    fn skip_newlines(&mut self) {
        while self.curr_token == Token::NewLine {
            self.next_token();
        }
    }

    fn skip_until(&mut self, t: Token) {
        while self.curr_token != t && t != Token::EOF {
            self.next_token();
        }
    }

    pub fn parse_section(&mut self) -> Result<Commit> {
        self.expect_token(Token::Commit);
        let Token::Hash(ref hash) = self.curr_token else {
            panic!("First token should be a hash, {:?}", self.curr_token);
        };
        let hash = hash.clone();
        self.next_token();

        let mut head = None;
        if self.curr_token == Token::LParen {
            head = Some(self.get_head());
        }
        self.skip_until(Token::Date);
        let date = self.get_date();

        self.skip_newlines();

        let mut message: Vec<String> = Vec::new(); // TODO: change this to a vec
        loop {
            match self.curr_token {
                Token::Word(ref s) => message.push(s.to_string()),
                Token::EOF => break,
                Token::Commit => {
                    // this checks if there is the word "commit" in the commit message
                    if let Token::Hash(_) = &self.peek_token {
                        break;
                    }
                    message.push("commit".to_string());
                }
                Token::NewLine => message.push("\n".to_string()),
                Token::Author => message.push("Author:".to_string()),
                Token::Date => message.push("Date:".to_string()),
                _ => panic!("this token should not be here... {:?}", self.curr_token),
            }
            self.next_token();
        }
        Ok(Commit {
            hash,
            head,
            message: message.join(" ").trim().into(),
            date,
        })
    }

    pub fn get_head(&mut self) -> Vec<String> {
        let mut v = Vec::new();
        self.next_token(); // move past the LParen that brought us here

        loop {
            match &self.curr_token {
                Token::Word(w) => v.push(w.clone()),
                Token::RParen => break,
                _ => {}
            }
            self.next_token();
        }
        self.next_token();

        v
    }

    fn get_date(&mut self) -> String {
        self.next_token(); // move past the date keyword
        let mut out = Vec::new();
        loop {
            match &self.curr_token {
                Token::Word(w) => out.push(w.clone()),
                Token::NewLine | Token::EOF => break,
                _ => {}
            }
            self.next_token();
        }
        out.join(" ")
    }

    pub fn parse_program(&mut self) -> Program {
        let mut commits = Vec::new();
        let mut errors = Vec::new();

        while self.curr_token != Token::EOF {
            match self.parse_section() {
                Ok(line) => commits.push(line),
                Err(err) => errors.push(err),
            }
        }

        Program(commits, errors)
    }
}

#[cfg(test)]
mod tests {
    use super::{Commit, Lexer, Parser, Program};

    fn compare_commits(a: &Commit, b: &Commit) {
        assert_eq!(a.hash, b.hash);
        assert_eq!(a.head, b.head);
        assert_eq!(a.message, b.message);
        assert_eq!(a.date, b.date);
    }

    fn compare_programs(a: &Program, b: Vec<Commit>) {
        assert_eq!(a.0.len(), b.len());
        a.0.iter()
            .zip(b.iter())
            .for_each(|(a, b)| compare_commits(a, b));
    }

    #[test]
    fn parsing_keywords_in_message() {
        let input = r#"commit bb4055c04da174bbfc93e63952d4ccc84e4832ab (HEAD -> master, origin/master)
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 17:52:39 2023 -0500

    HEAD commit Author: Date: testing
    "#;

        let commits = vec![Commit {
            hash: "bb4055c04da174bbfc93e63952d4ccc84e4832ab".to_string(),
            head: Some(vec![
                "HEAD".to_string(),
                "->".to_string(),
                "master,".to_string(),
                "origin/master".to_string(),
            ]),
            message: "HEAD commit Author: Date: testing".to_string(),
            date: "Sat Nov 25 17:52:39 2023 -0500".to_string(),
        }];

        let l = Lexer::new(input.to_string().into_bytes());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        compare_programs(&program, commits);
    }

    #[test]
    fn parsing_pretty_medium() {
        let input = r#"commit ebcbf7f96d2c6690e43833e60345075ce752bef0 (HEAD -> master)
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 22:56:43 2023 -0500

    feat: added date to matched commit output

commit bb4055c04da174bbfc93e63952d4ccc84e4832ab (origin/master)
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 17:52:39 2023 -0500

    feat: added parser for git log --pretty=medium as we want the date
    included

commit 0b5a4e8d5a1ae5b6d5539e3fc7023e0f3faf77af
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 15:58:03 2023 -0500

    feat: added target dir option"#;

        let commits = vec![
            Commit {
                hash: "ebcbf7f96d2c6690e43833e60345075ce752bef0".to_string(),
                head: Some(vec![
                    "HEAD".to_string(),
                    "->".to_string(),
                    "master".to_string(),
                ]),
                message: "feat: added date to matched commit output".to_string(),
                date: "Sat Nov 25 22:56:43 2023 -0500".to_string(),
            },
            Commit {
                hash: "bb4055c04da174bbfc93e63952d4ccc84e4832ab".to_string(),
                head: Some(vec!["origin/master".to_string()]),
                message:
                    "feat: added parser for git log --pretty=medium as we want the date \n included"
                        .to_string(),
                date: "Sat Nov 25 17:52:39 2023 -0500".to_string(),
            },
            Commit {
                hash: "0b5a4e8d5a1ae5b6d5539e3fc7023e0f3faf77af".to_string(),
                head: None,
                message: "feat: added target dir option".to_string(),
                date: "Sat Nov 25 15:58:03 2023 -0500".to_string(),
            },
        ];

        let l = Lexer::new(input.to_string().into_bytes());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        compare_programs(&program, commits);
    }
}
