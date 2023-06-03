use clap::{Parser};
use std::process::Command;

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    name: String,
}

fn main() {
    let cli = Cli::parse();
    let mut a = Command::new("git");
    a.arg("status");

    let o = a.output().expect("failed command");
    println!("Output: {:?} {:?}!", o, std::str::from_utf8(&o.stdout).unwrap());
}



