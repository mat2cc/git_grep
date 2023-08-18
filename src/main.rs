mod diff;
mod one_line;
mod matcher;
mod formatter;

use clap::Parser;
use std::{process::Command, time::Instant};

use one_line::lexer::Lexer;

use crate::matcher::{MatchFormat, do_the_matching};

#[derive(Parser, Clone)]
#[command(author, about, version)]
pub struct Cli {
    /// search string
    search: String,
    // path of the git repository
    // path: std::path::PathBuf,
    /// depth
    #[arg(short, long)]
    depth: Option<usize>, // TODO: context, we should be able to grab context from the standard diff
                          // however if context is too large, we might have to get the full diff rather than the
                          // compressed one

    /// Empty commits and files will not be printed
    #[arg(long)]
    ignore_empty: bool,

    #[arg(short = 'B', long)]
    before_context: Option<usize>,

    #[arg(short = 'A', long)]
    after_context: Option<usize>,

    #[arg(short = 'C', long)]
    context: Option<usize>,
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
    let matcher = do_the_matching(program, cli);

    println!("{}", matcher.print());
    println!("time elapsed: {}", now.elapsed().as_millis());
}
