pub trait ColorTrait {
    fn green(&self) -> String;
    fn red(&self) -> String;
    fn yellow(&self) -> String;
    fn cyan(&self) -> String;
}

pub trait StyleTrait {
    fn bold(&self) -> String;
    fn underline(&self) -> String;
    fn italic(&self) -> String;
}

impl StyleTrait for str {
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self)
    }
}

impl ColorTrait for str {
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }
}


impl ColorTrait for String {
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }
}

impl StyleTrait for String {
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self)
    }
}
