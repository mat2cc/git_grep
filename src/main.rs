mod diff;
mod one_line;
mod matcher;

use clap::Parser;
use std::process::Command;

use one_line::{lexer::Lexer, parser::Commit};

use crate::{diff::{diff_lexer::DiffLexer, diff_parser::DiffParser}, matcher::{Matcher, MatchFormat}};

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    /// search string
    search: String,
    // path of the git repository
    // path: std::path::PathBuf,
    /// depth
    #[arg(short, long)]
    depth: Option<usize>, // TODO: context, we should be able to grab context from the standard diff
                          // however if context is too large, we might have to get the full diff rather than the
                          // compressed one
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

    let matcher = Matcher::new(&program, &cli.search);

    // TODO: convert this to be multithreaded

    println!("{}", matcher.print());
}
