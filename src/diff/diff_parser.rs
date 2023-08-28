use std::sync::Arc;

use crate::{diff::diff_ast::ContentType, Options};
    diff_ast::{Chunk, Content, Program, Statement, ChunkStatement, LineStatement},
    fn parse_statement(&mut self, opts: Arc<Options>) -> Result<Statement, String> {
        match opts.format {
            crate::StatementType::Lines =>  {
                let mut lines = vec![];
                while self.curr_token == DiffToken::ChunkMarker {
                    lines.append(&mut self.parse_chunk()?.content.clone());
                }
                return Ok(Statement {
                    a_file,
                    b_file,
                    data: Box::new(LineStatement(lines)),
                });
            }
            crate::StatementType::Chunks => {
                let mut chunks = vec![];
                while self.curr_token == DiffToken::ChunkMarker {
                    chunks.push(self.parse_chunk()?);
                }

                return Ok(Statement {
                    a_file,
                    b_file,
                    data: Box::new(ChunkStatement(chunks)),
                });
            }
        }

    pub fn parse_program(&mut self, opts: Arc<Options>) -> Program {
            match self.parse_statement(opts.clone()) {
                    // println!("{:?}", program.statements);
// #[cfg(test)]
// mod tests {
//     use crate::diff::diff_ast::{Chunk, Content, ContentType, Statement};
// 
//     use super::{DiffLexer, DiffParser};
// 
//     #[test]
//     fn testing_incorrect_symbols() {
//         let input = r#"diff --git a/src/ast.rs b/src/ast.rs
// deleted file mode 100644
// index 318bd87..0000000
// --- a/src/ast.rs
// +++ /dev/null
// @@ -1,8 +0,0 @@
// -enum Ast { 
//     Testing // @@ a
// -} 
// @@ -10,80 +10,60 @@
// -enum Test {
// +   Hi
// -}
// "#;
// 
//         let match_statements = vec![Statement {
//             a_file: String::from("a/src/ast.rs"),
//             b_file: String::from("b/src/ast.rs"),
//             lines: vec![],
//             chunks: vec![
//                 Chunk {
//                     added_start: 0,
//                     added_changes: 0,
//                     removed_start: 1,
//                     removed_changes: 8,
//                     content: vec![
//                         Content {
//                             line_data: "enum Ast {".into(),
//                             c_type: ContentType::Remove,
//                         },
//                         Content {
//                             line_data: "Testing // @@ a".into(),
//                             c_type: ContentType::Neutral,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Remove,
//                         },
//                     ],
//                 },
//                 Chunk {
//                     added_start: 10,
//                     added_changes: 60,
//                     removed_start: 10,
//                     removed_changes: 80,
//                     content: vec![
//                         Content {
//                             line_data: "enum Test {".into(),
//                             c_type: ContentType::Remove,
//                         },
//                         Content {
//                             line_data: "Hi".into(),
//                             c_type: ContentType::Add,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Remove,
//                         },
//                     ],
//                 },
//             ],
//         }];
// 
//         let l = DiffLexer::new_from_string(input.into());
//         let mut t = DiffParser::new(l);
//         let p = t.parse_program();
// 
//         assert_eq!(p.errors.len(), 0);
//         assert_eq!(p.statements, match_statements);
//     }
//     #[test]
//     fn main_test() {
//         let input = r#"diff --git a/src/ast.rs b/src/ast.rs
// deleted file mode 100644
// index 318bd87..0000000
// --- a/src/ast.rs
// +++ /dev/null
// @@ -1,8 +0,0 @@
// -enum Ast {
// -} 
// diff --git a/src/diff/diff_ast.rs b/src/diff/diff_ast.rs
// new file mode 100644
// index 0000000..000012a
// --- /dev/null
// +++ b/src/diff/diff_ast.rs
// @@ -0,0 +1,33 @@
// +pub struct Program{
// +    pub statements: Vec<Statement>,
// +    pub errors: Vec<String>
// +}"#;
// 
//         let match_statements = vec![
//             Statement {
//                 a_file: String::from("a/src/ast.rs"),
//                 b_file: String::from("b/src/ast.rs"),
//                 lines: vec![],
//                 chunks: vec![Chunk {
//                     removed_start: 1,
//                     removed_changes: 8,
//                     added_start: 0,
//                     added_changes: 0,
//                     content: vec![
//                         Content {
//                             line_data: "enum Ast {".into(),
//                             c_type: ContentType::Remove,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Remove,
//                         },
//                     ],
//                 }],
//             },
//             Statement {
//                 a_file: String::from("a/src/diff/diff_ast.rs"),
//                 b_file: String::from("b/src/diff/diff_ast.rs"),
//                 lines: vec![],
//                 chunks: vec![Chunk {
//                     removed_start: 0,
//                     removed_changes: 0,
//                     added_start: 1,
//                     added_changes: 33,
//                     content: vec![
//                         Content {
//                             line_data: "pub struct Program{".into(),
//                             c_type: ContentType::Add,
//                         },
//                         Content {
//                             line_data: "pub statements: Vec<Statement> ,".into(),
//                             c_type: ContentType::Add,
//                         },
//                         Content {
//                             line_data: "pub errors: Vec<String>".into(),
//                             c_type: ContentType::Add,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Add,
//                         },
//                     ],
//                 }],
//             },
//         ];
// 
//         let l = DiffLexer::new_from_string(input.into());
//         let mut t = DiffParser::new(l);
//         let p = t.parse_program();
// 
//         assert_eq!(p.errors.len(), 0);
//         assert_eq!(p.statements, match_statements);
//     }
// 
//     #[test]
//     fn testing_multiple_chunks() {
//         let input = r#"diff --git a/src/ast.rs b/src/ast.rs
// deleted file mode 100644
// index 318bd87..0000000
// --- a/src/ast.rs
// +++ /dev/null
// @@ -1,8 +0,0 @@
// -enum Ast {
// -} 
// @@ -10,80 +10,60 @@
// -enum Test {
// +   Hi
// -}
// "#;
// 
//         let match_statements = vec![Statement {
//             a_file: String::from("a/src/ast.rs"),
//             b_file: String::from("b/src/ast.rs"),
//             lines: vec![],
//             chunks: vec![
//                 Chunk {
//                     added_start: 0,
//                     added_changes: 0,
//                     removed_start: 1,
//                     removed_changes: 8,
//                     content: vec![
//                         Content {
//                             line_data: "enum Ast {".into(),
//                             c_type: ContentType::Remove,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Remove,
//                         },
//                     ],
//                 },
//                 Chunk {
//                     added_start: 10,
//                     added_changes: 60,
//                     removed_start: 10,
//                     removed_changes: 80,
//                     content: vec![
//                         Content {
//                             line_data: "enum Test {".into(),
//                             c_type: ContentType::Remove,
//                         },
//                         Content {
//                             line_data: "Hi".into(),
//                             c_type: ContentType::Add,
//                         },
//                         Content {
//                             line_data: "}".into(),
//                             c_type: ContentType::Remove,
//                         },
//                     ],
//                 },
//             ],
//         }];
// 
//         let l = DiffLexer::new_from_string(input.into());
//         let mut t = DiffParser::new(l);
//         let p = t.parse_program();
// 
//         assert_eq!(p.errors.len(), 0);
//         assert_eq!(p.statements, match_statements);
//     }
// }