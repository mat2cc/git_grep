pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
    pub errors: Vec<String>
}

impl<'a> Program<'a> {
    pub fn new () -> Self {
        Self {
            statements: Vec::new(),
            errors: Vec::new()
        }
    }
}

enum ContentType {
    Add,
    Remove,
    Neutral
}

pub struct Content {
    line_data: String,
    c_type: ContentType
}

pub struct Chunk {}

pub struct Statement<'a> {
    a_file: &'a str,
    b_file: &'a str,
    pub chunks: Vec<Chunk>
}

