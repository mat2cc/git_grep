use std::fmt::Display;
#[derive(Debug, PartialEq)]
pub enum DiffToken {
    Word(String),
    Int(usize),

    Diff,
    Git,
    Index,
    ChunkMarker, // @@

    Comma,
    Dash,
    Plus,
    TripleDash,
    TriplePlus,
    NewLine,

    EOF,
    Illegal,
}

impl Display for DiffToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffToken::Word(w) => write!(f, "{}", w),
            DiffToken::Int(i) => write!(f, "{i}"),
            DiffToken::Diff => write!(f, "diff"),
            DiffToken::Git => write!(f, "--git"),
            DiffToken::Index => write!(f, "index"),
            DiffToken::ChunkMarker => write!(f, "@@"),
            DiffToken::Comma => write!(f, ","),
            DiffToken::Dash => write!(f, "-"),
            DiffToken::Plus => write!(f, "+"),
            DiffToken::TripleDash => write!(f, "---"),
            DiffToken::TriplePlus => write!(f, "+++"),
            DiffToken::NewLine => write!(f, "\n"),
            DiffToken::EOF => write!(f, "\0"),
            DiffToken::Illegal => write!(f, "ILLEGAL CHAR"),
        }
    }
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

    fn count_whitespace(&mut self) -> usize {
        let mut count = 0;
        while self.ch.is_ascii_whitespace() && self.ch != b'\n' {
            if self.ch == b'\t' {
                count += 4;
            } else {
                count += 1
            }
            self.read_char();
        }
        count
    }

    fn read_word(&mut self) -> DiffToken {
        let s_pos = self.pos;
        while !self.ch.is_ascii_whitespace() && self.ch != b'\0' && self.ch != b',' {
            self.read_char()
        }
        let word =
            std::str::from_utf8(&self.input[s_pos..self.pos]).expect("This should be valid ascii");
        let a = word.into();
        a
    }

    fn read_int(&mut self) -> DiffToken {
        let s_pos = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char()
        }
        let slice = &self.input[s_pos..self.pos];
        let slice = std::str::from_utf8(slice).unwrap();
        let int = slice.parse::<usize>().unwrap();

        return DiffToken::Int(int);
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

    fn is_entire_int(&self) -> bool {
        let mut pos = self.pos;
        loop {
            let ch = self.input[pos];
            if ch.is_ascii_digit() {
                pos += 1;
            } else if ch.is_ascii_alphabetic() {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn next_token(&mut self) -> (usize, DiffToken) {
        let ws = self.count_whitespace();

        let t = match self.ch {
            b'\0' => DiffToken::EOF,
            b'\n' => DiffToken::NewLine,
            b',' => DiffToken::Comma,
            b'-' => return (ws, self.match_dash()),
            b'+' => {
                if self.check_word("+++") {
                    self.read_multiple(3);
                    return (ws, DiffToken::TriplePlus);
                } else {
                    self.read_char();
                    return (ws, DiffToken::Plus);
                }
            }
            rest if rest.is_ascii_digit() => {
                if self.is_entire_int() {
                    return (ws, self.read_int());
                } else {
                    return (ws, self.read_word());
                }
            }
            rest if rest.is_ascii() => return (ws, self.read_word()),
            _ => DiffToken::Illegal,
        };

        self.read_char();
        (ws, t)
    }
}
#[cfg(test)]
mod tests {
    impl DiffLexer {
        pub fn new_from_string(s: String) -> Self {
            let input = s.into_bytes();
            Self {
                ch: input[0],
                input,
                pos: 0,
            }
        }
    }

    use super::{DiffLexer, DiffToken};

    #[test]
    fn testing_spacing() {
        let input = r#"-enum AST {
+use super::diff_ast::{Content, ContentType, Statement};
-  indentTwo
-    indentFour
-	tabIndent
}"#;

        use DiffToken::*;
        let output = vec![
            (0, Dash),
            (0, Word(String::from("enum"))),
            (1, Word(String::from("AST"))),
            (1, Word(String::from("{"))),
            (0, NewLine),
            (0, Plus),
            (0, Word(String::from("use"))),
            (1, Word(String::from("super::diff_ast::{Content"))),
            (0, Comma),
            (1, Word(String::from("ContentType"))),
            (0, Comma),
            (1, Word(String::from("Statement};"))),
            (0, NewLine),
            (0, Dash),
            (2, Word(String::from("indentTwo"))),
            (0, NewLine),
            (0, Dash),
            (4, Word(String::from("indentFour"))),
            (0, NewLine),
            (0, Dash),
            (4, Word(String::from("tabIndent"))),
            (0, NewLine),
            (0, Word(String::from("}"))),
        ];

        let mut l = DiffLexer::new_from_string(input.into());
        println!("testing tokens");
        for out in output {
            let token = l.next_token();
            assert_eq!(out, token);
        }
    }

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
            DiffToken::Int(100644),
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
            DiffToken::Int(1),
            DiffToken::Comma,
            DiffToken::Int(8),
            DiffToken::Plus,
            DiffToken::Int(0),
            DiffToken::Comma,
            DiffToken::Int(0),
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
            let (_, token) = l.next_token();
            assert_eq!(output[i], token);
        }
    }
}
