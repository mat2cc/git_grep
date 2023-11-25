use std::sync::Arc;

use crate::Options;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentType {
    Add,
    Remove,
    Neutral,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Neutral
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Add => "+".to_string(),
            ContentType::Remove => "-".to_string(),
            ContentType::Neutral => "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Content {
    pub line_data: String,
    pub c_type: ContentType,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub added_start: usize,
    pub added_changes: usize,
    pub removed_start: usize,
    pub removed_changes: usize,
    pub content: Vec<Content>,
}

/* STATEMENTS */

pub trait StatementTrait {
    fn fmt(&self, options: Arc<Options>) -> (String, usize);
    // TODO: add some print option here. OR we can try to implement Debug for implemented structs
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub a_file: String,
    pub b_file: String,
    pub data: Vec<Content>,
}
