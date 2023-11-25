    diff_ast::{Chunk, Content, Program, Statement},
use crate::diff::diff_ast::ContentType;
    pre_whitespace: usize,
    peek_whitespace: usize,
        let (_, curr_token) = l.next_token();
        let (_, peek_token) = l.next_token();
            pre_whitespace: 0,
            peek_whitespace: 0,
        self.pre_whitespace = std::mem::take(&mut self.peek_whitespace);
        (self.peek_whitespace, self.peek_token) = self.l.next_token();
            s.push(format!("{}", " ".repeat(self.pre_whitespace)));
            line_data: s.join(""),
                }
    fn skip_until_newline(&mut self) {
        while self.curr_token != DiffToken::NewLine && self.curr_token != DiffToken::EOF {
            self.next_token();
        }
    }

        self.skip_until_newline(); // don't include the chunk context as it's not part of the diff
    fn parse_statement(&mut self) -> Result<Statement, String> {
        let mut lines = vec![];
        while self.curr_token == DiffToken::ChunkMarker {
            lines.append(&mut self.parse_chunk()?.content.clone());
        return Ok(Statement {
            a_file,
            b_file,
            data: lines,
        });
    }
    pub fn parse_program(&mut self) -> Program {
            match self.parse_statement() {
#[cfg(test)]
mod tests {
    use crate::diff::diff_ast::{Content, ContentType, Statement};

    use super::{DiffLexer, DiffParser};

    #[test]
    fn parser_spacing_test() {
        let input = r#"diff --git a/src/ast.rs b/src/ast.rs
deleted file mode 100644
index 318bd87..0000000
--- a/src/ast.rs
+++ /dev/null
@@ -1,8 +0,0 @@
+use super::diff_ast::{Content, ContentType, Statement};
-  indentTwo
-    indentFour
-	tabIndent
}"#;
        use ContentType::*;
        let match_statements = vec![Statement {
            a_file: String::from("a/src/ast.rs"),
            b_file: String::from("b/src/ast.rs"),
            data: vec![
                Content {
                    line_data: String::from("use super::diff_ast::{Content, ContentType, Statement};"),
                    c_type: Add,
                },
                Content {
                    line_data: String::from("  indentTwo"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("    indentFour"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("    tabIndent"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("}"),
                    c_type: Neutral,
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
    fn happy_path() {
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
+    Hi
-}
"#;

        use ContentType::*;
        let match_statements = vec![Statement {
            a_file: String::from("a/src/ast.rs"),
            b_file: String::from("b/src/ast.rs"),
            data: vec![
                Content {
                    line_data: String::from("enum Ast {"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("     Testing // @@ a"), // 5 spaces here since the
                                                                     // content type is neutral
                    c_type: Neutral,
                },
                Content {
                    line_data: String::from("}"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("enum Test {"),
                    c_type: Remove,
                },
                Content {
                    line_data: String::from("    Hi"), // 4 spaces
                    c_type: Add,
                },
                Content {
                    line_data: String::from("}"),
                    c_type: Remove,
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

//
//
// -enum Ast {
// -}
//
//
//
// -}
//
//
//
//
// -}
//
//
//