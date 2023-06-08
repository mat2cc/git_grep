#![allow(dead_code)]
pub enum Diff {
    Add,
    Remove,
    Neutral,
}

struct DiffLine {
    diff: Diff,
    content: String,
}

#[derive(Debug, PartialEq)]
pub enum DiffToken {
    Word(String),

    Diff,
    Git,
    Index,
    ChunkMarker, // @@

    Dash,
    Plus,
    TripleDash,
    TriplePlus,
    NewLine,

    EOF,
    Illegal,
}

impl Default for DiffToken {
    fn default() -> Self {
        DiffToken::Illegal
    }
}

impl From<&str> for DiffToken {
    fn from(s: &str) -> Self {
        match s {
            "diff" => DiffToken::Diff,
            "index" => DiffToken::Index,
            "@@" => DiffToken::ChunkMarker,
            _ => DiffToken::Word(s.into()),
        }
    }
}

pub struct DiffLexer {
    input: Vec<u8>,
    pos: usize,
    ch: u8,
}

impl DiffLexer {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            ch: input[0],
            input,
            pos: 0,
        }
    }

    pub fn new_from_string(s: String) -> Self {
        let input = s.into_bytes();
        Self {
            ch: input[0],
            input,
            pos: 0,
        }
    }

    fn read_multiple(&mut self, n: usize) {
        for _ in 0..n {
            self.read_char();
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

    fn read_word(&mut self) -> DiffToken {
        let s_pos = self.pos;
        while !self.ch.is_ascii_whitespace() && self.ch != b'\0' {
            self.read_char()
        }
        let word =
            std::str::from_utf8(&self.input[s_pos..self.pos]).expect("This should be valid ascii");
        let a = word.into();
        a
    }

    fn check_word(&self, word: &str) -> bool {
        let mut offset = 0;
        let bytes = word.as_bytes();
        while self.ch != 0 && offset < word.len() {
            if self.input[self.pos + offset] != bytes[offset] {
                return false;
            }
            offset += 1;
        }
        return true;
    }

    pub fn match_dash(&mut self) -> DiffToken {
        if self.check_word("--git") {
            self.read_multiple(5);
            return DiffToken::Git;
        } else if self.check_word("---") {
            self.read_multiple(3);
            return DiffToken::TripleDash;
        } else {
            self.read_char();
            return DiffToken::Dash;
        }
    }

    pub fn next_token(&mut self) -> DiffToken {
        self.skip_whitespace();

        let t = match self.ch {
            b'\0' => DiffToken::EOF,
            b'\n' => DiffToken::NewLine,
            b'-' => return self.match_dash(),
            b'+' => {
                if self.check_word("+++") {
                    self.read_multiple(3);
                    return DiffToken::TriplePlus;
                } else {
                    self.read_char();
                    return DiffToken::Plus;
                }
            }
            rest if rest.is_ascii() => return self.read_word(),
            _ => DiffToken::Illegal,
        };

        self.read_char();
        t
    }
}
#[cfg(test)]
mod tests {
    use super::{DiffLexer, DiffToken};

    #[test]
    fn tokenize() {
        let input = r#"diff --git a/src/ast.rs b/src/ast.rs
deleted file mode 100644
index 318bd87..0000000
--- a/src/ast.rs
+++ /dev/null
@@ -1,8 +0,0 @@
-enum Ast {
-}
diff
"#;
        let output = vec![
            DiffToken::Diff,
            DiffToken::Git,
            DiffToken::Word(String::from("a/src/ast.rs")),
            DiffToken::Word(String::from("b/src/ast.rs")),
            DiffToken::NewLine,
            DiffToken::Word(String::from("deleted")),
            DiffToken::Word(String::from("file")),
            DiffToken::Word(String::from("mode")),
            DiffToken::Word(String::from("100644")),
            DiffToken::NewLine,
            DiffToken::Index,
            DiffToken::Word(String::from("318bd87..0000000")),
            DiffToken::NewLine,
            DiffToken::TripleDash,
            DiffToken::Word(String::from("a/src/ast.rs")),
            DiffToken::NewLine,
            DiffToken::TriplePlus,
            DiffToken::Word(String::from("/dev/null")),
            DiffToken::NewLine,
            DiffToken::ChunkMarker,
            DiffToken::Dash,
            DiffToken::Word(String::from("1,8")),
            DiffToken::Plus,
            DiffToken::Word(String::from("0,0")),
            DiffToken::ChunkMarker,
            DiffToken::NewLine,
            DiffToken::Dash,
            DiffToken::Word(String::from("enum")),
            DiffToken::Word(String::from("Ast")),
            DiffToken::Word(String::from("{")),
            DiffToken::NewLine,
            DiffToken::Dash,
            DiffToken::Word(String::from("}")),
            DiffToken::NewLine,
            DiffToken::Diff,
            DiffToken::NewLine,
        ];

        let mut l = DiffLexer::new_from_string(input.into());
        for i in 0..output.len() {
            // println!("{:?}", l.next_token())
            assert_eq!(output[i], l.next_token())
        }
    }
}
