mod diff;
mod matcher;
mod pretty_medium;
mod formatter;

use clap::{Parser as ClapParser, ValueEnum};
use std::{process::Command, time::Instant};

use pretty_medium::{lexer::Lexer, parser::Parser};

use crate::matcher::{do_the_matching, MatchFormat};

#[derive(ValueEnum, Clone, Debug)]
enum ColorInput {
    Auto,
    Colored,
    Uncolored,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSettings {
    Colored,
    Uncolored,
}

#[derive(ClapParser)]
#[command(author, about, version)]
struct Cli {
    /// search string
    search: String,

    /// depth
    #[arg(short = 'D', long)]
    depth: Option<usize>,

    /// Empty commits and files will be printed
    #[arg(long)]
    show_empty: bool,

    /// print NUM lines of leading context
    #[arg(short = 'B', long)]
    before_context: Option<usize>,
    /// print NUM lines of trailing context
    #[arg(short = 'A', long)]
    after_context: Option<usize>,
    /// print NUM lines of output context
    #[arg(short = 'C', long)]
    context: Option<usize>,

    /// do not print the file name and the number of matches per file
    #[arg(long)]
    skip_file_print: bool,

    /// git directory to search in
    #[arg(long)]
    target_dir: Option<String>,

    #[arg(long, default_value = "auto")]
    color: Option<ColorInput>,

    /// print only the matched lines, with their commit and file diffs
    #[arg(long)]
    simple_print: bool,
}

impl Into<ColorSettings> for ColorInput {
    fn into(self) -> ColorSettings {
        match self {
            ColorInput::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    ColorSettings::Colored
                } else {
                    ColorSettings::Uncolored
                }
            }
            ColorInput::Colored => ColorSettings::Colored,
            ColorInput::Uncolored => ColorSettings::Uncolored,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    before_context: usize,
    after_context: usize,
    show_empty: bool,
    search_string: String,
    skip_file_print: bool,
    target_dir: Option<String>,
    color: ColorSettings,
}

impl From<Cli> for Options {
    fn from(cli: Cli) -> Self {
        Self {
            search_string: cli.search,
            before_context: cli.before_context.unwrap_or(cli.context.unwrap_or(0)),
            after_context: cli.after_context.unwrap_or(cli.context.unwrap_or(0)),
            show_empty: cli.show_empty,
            skip_file_print: cli.skip_file_print,
            target_dir: cli.target_dir,
            color: cli.color.unwrap_or(ColorInput::Auto).into(),
        }
    }
}

fn main() {
    let now = Instant::now();
    let cli = Cli::parse();
    let mut a = Command::new("git");
    a.arg("log");
    a.arg("--pretty=medium");
    if let Some(target_dir) = &cli.target_dir {
        a.current_dir(target_dir);
    }
    if let Some(depth) = cli.depth {
        if depth == 0 {
            panic!("depth must be greater than 0");
        }
        a.args(["-n", &(depth + 1).to_string()]); // +1 because the first line is the HEAD
    }

    let o = a.output().expect("failed command");

    let l = Lexer::new(o.stdout);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    let simple_print = cli.simple_print;

    let options = Options::from(cli);
    let matcher = do_the_matching(program, options.clone());

    if simple_print {
        println!("{}", matcher.simple_print(options));
    } else {
        println!("{}", matcher.print(options));
    }
    eprintln!("time elapsed: {}", now.elapsed().as_millis());
}
