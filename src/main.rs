mod diff;
mod one_line;

use clap::Parser;
use std::process::Command;

use one_line::lexer::Lexer;


#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    /// search string
    search: String,
    // path of the git repository
    // path: std::path::PathBuf,

    /// depth
    #[arg(short, long)]
    depth: Option<usize> 

    // TODO: context, we should be able to grab context from the standard diff
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

    let mut out = String::new();
    for x in program.0.iter() {
        let diff = std::process::Command::new("git")
            .args(["diff", &x.hash])
            .output()
            .expect(&format!("failed diff for commit {}", &x.hash));
        let str_diff = std::str::from_utf8(&diff.stdout)
            .expect("couldn't read file");

        if str_diff.contains(&cli.search) {
            out.push_str(&format!("Commit: {}\n", &x.hash));
            str_diff
                .lines()
                .filter(|l| l.contains(&cli.search))
                .for_each(|l| out.push_str(&format!("{}\n", l)));
        }
    }

    println!("{}", program.print());
    println!("{}", out);
}
