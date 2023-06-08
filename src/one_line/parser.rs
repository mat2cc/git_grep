use super::lexer::{Lexer, Token};

pub struct Program(pub Vec<Commit>);

impl Program {
    pub fn print(&self) -> String {
        let mut out = String::new();
        for c in &self.0 {
            out.push_str(&c.print());
        }
        out
    }
}

pub struct Commit {
    pub hash: String,
    message: String,
    head: Option<Vec<String>>,
}

impl Commit {
    pub fn print(&self) -> String {
        let head = self.head.clone().unwrap_or_default();
        let mut head = head.join(" ");
        if !head.is_empty() {
            head = format!("  (HEAD -> {})", head);
        }
        format!("{}{} {}", self.hash, head, self.message)
    }
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

    pub fn parse_line(&mut self) -> Commit {
        let Token::Hash(ref hash) = self.curr_token else {
            panic!("First token should be a hash");
        };
        let hash = hash.clone();
        self.next_token();

        let mut head = None;
        if self.curr_token == Token::LParen {
            head = Some(self.get_head());
        }

        let mut message = String::new();
        loop {
            match &self.curr_token {
                Token::Word(s) => {
                    message.push_str(&format!(" {}", s));
                    self.next_token();
                }
                Token::NewLine => {
                    message.push_str("\n");
                    self.next_token();
                }
                _ => break,
            }
        }
        Commit {
            hash,
            head,
            message,
        }
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
                _ => self.next_token()
            }
        }
        self.next_token();

        v
    }

    pub fn parse_program(&mut self) -> Program {
        let mut commits = Vec::new();

        while self.curr_token != Token::EOF {
            let line = self.parse_line();
            commits.push(line);
        }

        Program(commits)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Parser};

    #[test]
    fn parsing() {
        let input = r#"c03b7c35a902784aae4c3fd7fdfb8479734fda70 (HEAD -> another_test, test, master) Title
a42cc2e1c21d71ce016b5b878b4b1ac801a5fb83 feat: parsing done
55207669734163e6e983160ceb24248a5428505d feat: if/else
127e8399a31deb526bf5686029f3ff9addf8c9d7 feat: completed up to and including booleans
b132603284b4de2895031cf3eaa118f746abb13a feat: ast implemented
16d7df69c7e3d102952dcde48a2a85a663710431 interepreter"#;

        let l = Lexer::new_from_string(input.clone().into());
        // let mut lex2 = Lexer::new(input.into());
        // loop {
        //     match lex2.next_token() {
        //         Token::EOF => break,
        //         x => println!("{:?}", x)
        //     }
        // }
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{}", program.print());
    }
}
