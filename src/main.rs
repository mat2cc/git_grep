mod diff;
mod one_line;
mod matcher;
mod formatter;

use clap::{Parser, ValueEnum};
use std::{process::Command, time::Instant};

use one_line::lexer::Lexer;

use crate::matcher::{MatchFormat, do_the_matching};

#[derive(ValueEnum, Clone, Debug)]
enum StatementType {
    Lines,
    Chunks
}

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    /// search string
    search: String,
    // path of the git repository
    // path: std::path::PathBuf,
    /// depth
    #[arg(short = 'D', long)]
    depth: Option<usize>,

    /// Empty commits and files will be printed
    #[arg(long)]
    show_empty: bool,

    #[arg(short = 'B', long)]
    before_context: Option<usize>,

    #[arg(short = 'A', long)]
    after_context: Option<usize>,

    #[arg(short = 'C', long)]
    context: Option<usize>,

    #[arg(value_enum, long, default_value = "lines")]
    format: StatementType,

    /// do not print the file name and the number of matches per file
    #[arg(long)]
    skip_file_print: bool
}

#[derive(Debug, Clone)]
pub struct Options {
    before_context: usize,
    after_context: usize,
    show_empty: bool,
    search_string: String,
    format: StatementType,
    skip_file_print: bool,
}

impl From<Cli> for Options {
    fn from(cli: Cli) -> Self {
        Self {
            search_string: cli.search,
            before_context: cli.before_context.unwrap_or(cli.context.unwrap_or(0)),
            after_context: cli.after_context.unwrap_or(cli.context.unwrap_or(0)),
            show_empty: cli.show_empty,
            format: cli.format,
            skip_file_print: cli.skip_file_print,
        }
    }
}

fn main() {
    let now = Instant::now();
    let cli = Cli::parse();
    let mut a = Command::new("git");
    a.arg("log");
    a.arg("--pretty=oneline");
    if let Some(depth) = cli.depth {
        if depth == 0 {
            panic!("depth must be greater than 0");
        }
        a.args(["-n", &(depth + 1).to_string()]); // +1 because the first line is the HEAD
    }

    let o = a.output().expect("failed command");
    let l = Lexer::new(o.stdout);
    let mut p = one_line::parser::Parser::new(l);
    let program = p.parse_program();

    let options = Options::from(cli);
    let matcher = do_the_matching(program, options.clone());

    println!("{}", matcher.print(options));
    println!("time elapsed: {}", now.elapsed().as_millis());
}
