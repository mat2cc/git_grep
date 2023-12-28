use crate::ColorSettings;

#[derive(Clone, Debug)]
pub enum Color {
    Green,
    Red,
    Yellow,
    Cyan,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Green => "\x1b[32m",
            Color::Red => "\x1b[31m",
            Color::Yellow => "\x1b[33m",
            Color::Cyan => "\x1b[36m",
        }
        .to_string()
    }
}

#[derive(Clone, Debug)]
pub enum Styles {
    Bold,
    Underline,
    Italic,
    Color(Color),
}

impl ToString for Styles {
    fn to_string(&self) -> String {
        match self {
            Styles::Bold => "\x1b[1m".to_string(),
            Styles::Underline => "\x1b[4m".to_string(),
            Styles::Italic => "\x1b[3m".to_string(),
            Styles::Color(color) => color.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StyleBuilder {
    color: ColorSettings,
    styles: Vec<Styles>,
}

impl StyleBuilder {
    pub fn new(color: ColorSettings) -> Self {
        Self {
            color,
            styles: Vec::new(),
        }
    }

    pub fn add_style(mut self, style: Styles) -> Self {
        self.styles.push(style);
        self
    }

    pub fn build(&self, input: &str) -> String {
        if self.color == ColorSettings::Uncolored {
            return input.to_string();
        }

        let mut out = String::new();
        for style in &self.styles {
            out.push_str(&style.to_string());
        }
        out.push_str(input);
        out.push_str("\x1b[0m");
        out
    }
}


