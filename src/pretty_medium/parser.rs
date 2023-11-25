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
                Token::Word(ref s) => {
                    message.push(s.to_string());
                    self.next_token();
                }
                Token::EOF => break,
                Token::Commit => {
                    // this checks if there is the word "commit" in the commit message
                    if let Token::Hash(_) = &self.peek_token {
                        break
                    }
                    message.push("commit".to_string());
                    self.next_token();
                }
                Token::NewLine => {
                    message.push("\n".to_string());
                    self.next_token();
                }
                Token::Head => message.push("HEAD".to_string()),
                Token::Author => message.push("Author:".to_string()),
                Token::Date => message.push("Date:".to_string()),
                _ => panic!("this token should not be here... {:?}", self.curr_token)
            }
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
        if Token::Head != self.curr_token {
            panic!("should be head?");
        };
        self.next_token();

        loop {
            match &self.curr_token {
                Token::Word(w) => {
                    v.push(w.clone());
                    self.next_token()
                }
                Token::RParen => break,
                _ => self.next_token(),
            }
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

    impl Program {
        pub fn print(&self) -> String {
            let mut out = String::new();
            for c in &self.0 {
                out.push_str(&c.print());
                out.push_str("\n");
            }
            out
        }
    }

    impl Commit {
        pub fn print(&self) -> String {
            let head = self.head.clone().unwrap_or_default();
            let mut head = head.join(" ");
            if !head.is_empty() {
                head = format!("  (HEAD -> {})", head);
            }
            format!(
                "out: {}{} - {} - {}",
                self.hash, head, self.message, self.date
            )
        }
    }

    #[test]
    fn parsing_pretty_medium() {
        let input = r#"commit 0b5a4e8d5a1ae5b6d5539e3fc7023e0f3faf77af (HEAD -> master, origin/master)
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 15:58:03 2023 -0500

    feat: added target dir option

commit 42541148c39c6479c0978663c99a718acbb43518
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Nov 25 12:28:31 2023 -0500

    cleanup up spacing and indentation WIP

commit 1e6760da86b9b4c473daec5256c2e5376456df27
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Tue Sep 5 23:41:26 2023 -0400

    feat: consolidated diff ast formatting, fixed order of commits

commit 0b898178746d6d5961c668845d36aa2ab2bc10fe
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Tue Sep 5 22:26:37 2023 -0400

    fix: correcting depth number, also cleanup deps

commit d3f7544757b9e727addc9e16abb6a6eed277b9f1
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Tue Sep 5 22:01:16 2023 -0400

    feat: include comparison commit in message

commit 3a392d66325ff47b41e67920e9909591ff054742
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Mon Sep 4 21:32:48 2023 -0400

    chore: cleanup and formatting

commit a77bffeba761a3fc4afc9e1ee620046e12dd3876
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Mon Aug 28 23:16:27 2023 -0400

    feat: re-wrote and simplified formatter. cleanup

commit fe65b612ecd0d16b53c21c71f2fe26bf99ccf2ee
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Mon Aug 28 18:01:33 2023 -0400

    feat: major fix to the git diff command to compare to last commit,
    added/implemented new options

commit a92cea8ab87fbd46f532161899bca96338d9c297
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Thu Aug 24 23:57:27 2023 -0400

    feat: added the option to parse diff by lines. Tests need fixing

commit 1a60fe2b81ec5d9ea1c9bdd80e9725c71ed291b7
Author: Matt Christofides <matt.christofides@gmail.com>
Date:   Sat Aug 19 14:32:59 2023 -0400

    refactor: created args object to be passed down, get context form git
    diff

"#;

        let l = Lexer::new(input.to_string().into_bytes());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{}", program.print());
    }
}
