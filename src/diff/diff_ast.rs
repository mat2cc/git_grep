use std::sync::Arc;
use std::boxed::Box;

use crate::Options;

pub struct Program{
    pub statements: Vec<Statement>,
    pub errors: Vec<String>
}

impl Program {
    pub fn new () -> Self {
        Self {
            statements: Vec::new(),
            errors: Vec::new()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentType {
    Add,
    Remove,
    Neutral
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Content {
    pub line_data: String,
    pub c_type: ContentType
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub added_start: usize,
    pub added_changes: usize,
    pub removed_start: usize,
    pub removed_changes: usize,
    pub content: Vec<Content>
}

/* STATEMENTS */

pub trait StatementTrait {
    fn fmt(&self, options: Arc<Options>) -> (String, usize);
    // TODO: add some print option here. OR we can try to implement Debug for implemented structs
}

// #[derive(Debug, PartialEq)]
pub struct Statement{
    pub a_file: String,
    pub b_file: String,
    pub data: Box<dyn StatementTrait>
}

#[derive(Debug, PartialEq)]
pub struct ChunkStatement(pub Vec<Chunk>);

impl StatementTrait for ChunkStatement {
    fn fmt(&self, options: Arc<Options>) -> (String, usize) {
        let mut out = String::new();
        let mut matched_lines = 0;
        for chunk in &self.0 {
            if chunk.content.len() == 0 {
                continue;
            }

            for content in &chunk.content {
                if content.line_data.contains(&options.search_string) {
                    out.push_str(&format!("{}\n", content.line_data));
                    matched_lines += 1;
                }
            }
        }
        (out, matched_lines)
    }
}

pub struct LineStatement(pub Vec<Content>);

impl StatementTrait for LineStatement {
    fn fmt(&self, options: Arc<Options>) -> (String, usize) {
        let mut out = String::new();
        let mut matched_lines = 0;
        let mut lines = vec![false; self.0.len()];
        for (idx, l) in self.0.iter().enumerate() {
            if l.line_data.contains(&options.search_string) {
                matched_lines += 1;
                add_context(&mut lines, idx, options.before_context, options.after_context);
            }
        }

        for x in 0..self.0.len() {
            if lines[x] {
                out.push_str(&format!("{}\n", &self.0[x].line_data));
                // add a spacer when we have reached a break in context
                if x + 1 < self.0.len() && !lines[x + 1] {
                    out.push_str("---\n");
                }
            }
        }

        (out, matched_lines)
    }
}

fn add_context(mut lines: &mut Vec<bool>, idx: usize, pre_context: usize, post_context: usize) {
    lines[idx] = true;
    if pre_context > 0 && idx > 0 { // make sure we have more pre-context, and we stay in bounds
        add_context(&mut lines, idx - 1, pre_context - 1, post_context);
    }
    if post_context > 0 && idx + 1 < lines.len() { // make sure we have more post-context, and we stay in bounds
        add_context(&mut lines, idx + 1, pre_context, post_context - 1);
    }
}

