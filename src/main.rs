mod parser;
mod lexer;
mod ast;
use clap::{Parser};
use std::process::Command;

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    search: String,
}

fn main() {
    let cli = Cli::parse();
    let mut a = Command::new("git");
    a.arg("log");
    a.arg("--pretty=oneline");

    let o = a.output().expect("failed command");
    println!("Output: {:?} {:?}!", o, std::str::from_utf8(&o.stdout).unwrap());
}

