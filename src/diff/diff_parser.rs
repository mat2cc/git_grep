use crate::diff::diff_ast::ContentType;

use super::{
    diff_ast::{Chunk, Content, Program, Statement},
    diff_lexer::{DiffLexer, DiffToken},
};

enum Error {
    ExpectTokenError(String),
}

pub struct DiffParser {
    l: DiffLexer,
    curr_token: DiffToken,
    peek_token: DiffToken,
}

impl DiffParser {
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
        while self.curr_token != t && self.curr_token != DiffToken::EOF {
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

    fn parse_diff_chunk_range(&mut self) -> Result<(usize, usize), String> {
        let DiffToken::Int(start) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        self.next_token();
        self.expect_token(DiffToken::Comma)?;
        let DiffToken::Int(changes) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        self.next_token();
        Ok((start, changes))
    }

    fn parse_content_line(&mut self) -> Option<Content> {
        let mut s = Vec::new();

        let c_type = match self.curr_token {
            DiffToken::Plus => {
                self.next_token();
                ContentType::Add
            }
            DiffToken::Dash => {
                self.next_token();
                ContentType::Remove
            }
            _ => ContentType::Neutral,
        };

        while self.curr_token != DiffToken::NewLine
            && self.curr_token != DiffToken::EOF
            && !(self.curr_token == DiffToken::ChunkMarker && self.peek_token == DiffToken::Dash)
            && !(self.curr_token == DiffToken::Diff && self.peek_token == DiffToken::Git)
        {
            let sub_str = self.curr_token.to_string();
            if !sub_str.is_empty() {
                s.push(sub_str);
            }
            self.next_token()
        }
        if s.is_empty() {
            return None;
        }

        return Some(Content {
            line_data: s.join(" "),
            c_type,
        });
    }

    fn parse_content(&mut self) -> Vec<Content> {
        let mut content_list = Vec::new();

        use DiffToken::*;
        loop {
            let content = match self.curr_token {
                Diff => {
                    if self.peek_token == Git {
                        break;
                    }
                    self.next_token();
                    self.parse_content_line()
                },
                EOF => break,
                ChunkMarker => break,
                NewLine => {
                    self.next_token();
                    self.parse_content_line()
                }
                _ => self.parse_content_line(),
            };
            if let Some(c) = content {
                content_list.push(c);
            }
        }

        content_list
    }

    fn parse_chunk(&mut self) -> Result<Chunk, String> {
        self.expect_token(DiffToken::ChunkMarker)?;
        self.expect_token(DiffToken::Dash)?;
        let (removed_start, removed_changes) = self.parse_diff_chunk_range()?;
        self.expect_token(DiffToken::Plus)?;
        let (added_start, added_changes) = self.parse_diff_chunk_range()?;
        self.expect_token(DiffToken::ChunkMarker)?;

        let content = self.parse_content();

        Ok(Chunk {
            added_start,
            added_changes,
            removed_start,
            removed_changes,
            content,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.expect_token(DiffToken::Diff)?;
        self.expect_token(DiffToken::Git)?;
        let DiffToken::Word(ref a_file) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        let a_file = a_file.clone();
        self.next_token();
        let DiffToken::Word(ref b_file) = self.curr_token else {
            return Err(format!("could not match word for token: {:?}", self.curr_token));
        };
        let b_file = b_file.clone();
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
                Err(e) => {
                    println!("{:?}", program.statements);
                    panic!("{e:?}");
                    // program.errors.push(e);
                }
            }
        }

        program
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::diff_ast::{Chunk, Content, ContentType, Statement};

    use super::{DiffLexer, DiffParser};

    #[test]
    fn testing_incorrect_symbols() {
        let input = r#"diff --git a/src/ast.rs b/src/ast.rs
deleted file mode 100644
index 318bd87..0000000
--- a/src/ast.rs
+++ /dev/null
@@ -1,8 +0,0 @@
-enum Ast { 
    Testing // @@ a
-} 
@@ -10,80 +10,60 @@
-enum Test {
+   Hi
-}
"#;

        let match_statements = vec![Statement {
            a_file: String::from("a/src/ast.rs"),
            b_file: String::from("b/src/ast.rs"),
            chunks: vec![
                Chunk {
                    added_start: 0,
                    added_changes: 0,
                    removed_start: 1,
                    removed_changes: 8,
                    content: vec![
                        Content {
                            line_data: "enum Ast {".into(),
                            c_type: ContentType::Remove,
                        },
                        Content {
                            line_data: "Testing // @@ a".into(),
                            c_type: ContentType::Neutral,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Remove,
                        },
                    ],
                },
                Chunk {
                    added_start: 10,
                    added_changes: 60,
                    removed_start: 10,
                    removed_changes: 80,
                    content: vec![
                        Content {
                            line_data: "enum Test {".into(),
                            c_type: ContentType::Remove,
                        },
                        Content {
                            line_data: "Hi".into(),
                            c_type: ContentType::Add,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Remove,
                        },
                    ],
                },
            ],
        }];

        let l = DiffLexer::new_from_string(input.into());
        let mut t = DiffParser::new(l);
        let p = t.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(p.statements, match_statements);
    }
    #[test]
    fn main_test() {
        let input = r#"diff --git a/src/ast.rs b/src/ast.rs
deleted file mode 100644
index 318bd87..0000000
--- a/src/ast.rs
+++ /dev/null
@@ -1,8 +0,0 @@
-enum Ast {
-} 
diff --git a/src/diff/diff_ast.rs b/src/diff/diff_ast.rs
new file mode 100644
index 0000000..000012a
--- /dev/null
+++ b/src/diff/diff_ast.rs
@@ -0,0 +1,33 @@
+pub struct Program{
+    pub statements: Vec<Statement>,
+    pub errors: Vec<String>
+}"#;

        let match_statements = vec![
            Statement {
                a_file: String::from("a/src/ast.rs"),
                b_file: String::from("b/src/ast.rs"),
                chunks: vec![Chunk {
                    removed_start: 1,
                    removed_changes: 8,
                    added_start: 0,
                    added_changes: 0,
                    content: vec![
                        Content {
                            line_data: "enum Ast {".into(),
                            c_type: ContentType::Remove,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Remove,
                        },
                    ],
                }],
            },
            Statement {
                a_file: String::from("a/src/diff/diff_ast.rs"),
                b_file: String::from("b/src/diff/diff_ast.rs"),
                chunks: vec![Chunk {
                    removed_start: 0,
                    removed_changes: 0,
                    added_start: 1,
                    added_changes: 33,
                    content: vec![
                        Content {
                            line_data: "pub struct Program{".into(),
                            c_type: ContentType::Add,
                        },
                        Content {
                            line_data: "pub statements: Vec<Statement> ,".into(),
                            c_type: ContentType::Add,
                        },
                        Content {
                            line_data: "pub errors: Vec<String>".into(),
                            c_type: ContentType::Add,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Add,
                        },
                    ],
                }],
            },
        ];

        let l = DiffLexer::new_from_string(input.into());
        let mut t = DiffParser::new(l);
        let p = t.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(p.statements, match_statements);
    }

    #[test]
    fn testing_multiple_chunks() {
        let input = r#"diff --git a/src/ast.rs b/src/ast.rs
deleted file mode 100644
index 318bd87..0000000
--- a/src/ast.rs
+++ /dev/null
@@ -1,8 +0,0 @@
-enum Ast {
-} 
@@ -10,80 +10,60 @@
-enum Test {
+   Hi
-}
"#;

        let match_statements = vec![Statement {
            a_file: String::from("a/src/ast.rs"),
            b_file: String::from("b/src/ast.rs"),
            chunks: vec![
                Chunk {
                    added_start: 0,
                    added_changes: 0,
                    removed_start: 1,
                    removed_changes: 8,
                    content: vec![
                        Content {
                            line_data: "enum Ast {".into(),
                            c_type: ContentType::Remove,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Remove,
                        },
                    ],
                },
                Chunk {
                    added_start: 10,
                    added_changes: 60,
                    removed_start: 10,
                    removed_changes: 80,
                    content: vec![
                        Content {
                            line_data: "enum Test {".into(),
                            c_type: ContentType::Remove,
                        },
                        Content {
                            line_data: "Hi".into(),
                            c_type: ContentType::Add,
                        },
                        Content {
                            line_data: "}".into(),
                            c_type: ContentType::Remove,
                        },
                    ],
                },
            ],
        }];

        let l = DiffLexer::new_from_string(input.into());
        let mut t = DiffParser::new(l);
        let p = t.parse_program();

        assert_eq!(p.errors.len(), 0);
        assert_eq!(p.statements, match_statements);
    }
}
