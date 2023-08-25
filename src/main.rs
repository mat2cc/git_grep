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
    #[arg(short, long)]
    depth: Option<usize>,

    /// Empty commits and files will not be printed
    #[arg(long)]
    ignore_empty: bool,

    #[arg(short = 'B', long)]
    before_context: Option<usize>,

    #[arg(short = 'A', long)]
    after_context: Option<usize>,

    #[arg(short = 'C', long)]
    context: Option<usize>,

    #[arg(value_enum, long)]
    format: StatementType,
}

#[derive(Debug, Clone)]
pub struct Options {
    before_context: usize,
    after_context: usize,
    ignore_empty: bool,
    search_string: String,
    format: StatementType,
}

impl From<Cli> for Options {
    fn from(cli: Cli) -> Self {
        Self {
            search_string: cli.search,
            before_context: cli.before_context.unwrap_or(cli.context.unwrap_or(0)),
            after_context: cli.after_context.unwrap_or(cli.context.unwrap_or(0)),
            ignore_empty: cli.ignore_empty,
            format: cli.format,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut a = Command::new("git");
    a.arg("log");
    a.arg("--pretty=oneline");
    if let Some(depth) = cli.depth {
        a.args(["-n", &depth.to_string()]);
    }

    let o = a.output().expect("failed command");
    let l = Lexer::new(o.stdout);
    let mut p = one_line::parser::Parser::new(l);
    let program = p.parse_program();

    let now = Instant::now();
    let matcher = do_the_matching(program, Options::from(cli));

    println!("{}", matcher.print());
    println!("time elapsed: {}", now.elapsed().as_millis());
}
