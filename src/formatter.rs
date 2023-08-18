#![allow(dead_code)]

#[derive(Debug)]
pub struct FormatBuilder<'a> {
    input: &'a str,
    foreground: Option<Color>,
    background: Option<Color>,
    styles: Option<Vec<Styles>>,
}

#[derive(Debug)]
pub enum Color {
    Green,
    Yellow,
    Red,
    Cyan,
}

impl Color {
    fn fg(self) -> &'static str {
        match self {
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Red => "31",
            Color::Cyan => "36",
        }
    }

    fn bg(self) -> &'static str {
        match self {
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Red => "41",
            Color::Cyan => "46",
        }
    }
}

#[derive(Debug)]
pub enum Styles {
    Bold,
    Italic,
    Underline,
}

impl Styles {
    fn fmt(self) -> &'static str {
        match self {
            Styles::Bold => "1",
            Styles::Italic => "3",
            Styles::Underline => "4",
        }
    }
}

impl<'a> FormatBuilder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            foreground: None,
            background: None,
            styles: None,
        }
    }

    pub fn add_style(mut self, new_style: Styles) -> Self {
        if let Some(mut styles) = self.styles {
            styles.push(new_style);
            self.styles = Some(styles);
        } else {
            self.styles = Some(vec![new_style]);
        }
        self
    }

    pub fn set_styles(mut self, styles: Vec<Styles>) -> Self {
        self.styles = Some(styles);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn build(self) -> String {
        if self.foreground.is_none() && self.background.is_none() && self.styles.is_none() {
            return self.input.to_string();
        }

        let mut initialized = false;
        let mut prefix = String::new();

        if let Some(styles) = self.styles {
            initialized = true;
            prefix.push_str(&styles.into_iter().map(|s| s.fmt()).collect::<Vec<_>>().join(";"));
        }

        if let Some(f) = self.foreground {
            if initialized {
                prefix.push(';');
            } else {
                initialized = true;
            }
            prefix.push_str(f.fg());
        }

        if let Some(b) = self.background {
            if initialized {
                prefix.push(';');
            }
            prefix.push_str(b.bg());
        }

        format!("\x1b[{}m{}\x1b[0m", prefix, self.input)
    }
}

trait PrettyFormat {
    fn bold(&self) -> String;
}

impl PrettyFormat for str {
    fn bold(&self) -> String {
        format!("\x1b[1")
    }
}
