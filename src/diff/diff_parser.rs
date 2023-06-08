use super::{
    diff_ast::{Program, Statement, Chunk},
    diff_lexer::{DiffLexer, DiffToken},
};

enum Error {
    ExpectTokenError(String),
}

pub struct Parser {
    l: DiffLexer,
    curr_token: DiffToken,
    peek_token: DiffToken,
}

impl Parser {
    pub fn new(mut l: DiffLexer) -> Self {
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

    fn skip_until(&mut self, t: DiffToken) {
        while self.curr_token != t || self.curr_token != DiffToken::EOF {
            self.next_token();
        }
    }

    fn expect_token(&mut self, t: DiffToken) -> Result<(), String> {
        if self.curr_token == t {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "got token: {:?}, expected token: {:?}",
                self.curr_token, t
            ))
        }
    }

    fn parse_chunk(&mut self) -> Result<Chunk, String> {
        self.expect_token(DiffToken::Dash)?;
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        self.expect_token(DiffToken::Diff)?;
        self.expect_token(DiffToken::Git)?;
        let DiffToken::Word(ref a_file) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        let DiffToken::Word(ref b_file) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        let mut s = Statement {
            a_file,
            b_file,
            chunks: Vec::new(),
        };

        self.skip_until(DiffToken::ChunkMarker);
        while self.curr_token == DiffToken::ChunkMarker {
            s.chunks.push(self.parse_chunk()?);
        }

        Ok(s)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.curr_token != DiffToken::EOF {
            match self.parse_statement() {
                Ok(s) => program.statements.push(s),
                Err(e) => program.errors.push(e),
            }
        }

        program
    }
}

#[cfg(test)]
mod tests {
    use super::{DiffLexer, Parser};

    #[test]
    fn parsing() {}
}
