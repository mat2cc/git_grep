#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Statement{
    pub a_file: String,
    pub b_file: String,
    pub chunks: Vec<Chunk>
}

