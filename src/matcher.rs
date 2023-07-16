use std::{thread, sync::{Arc, self, mpsc::Sender}, rc::Rc};

use crate::{
    diff::{diff_lexer::DiffLexer, diff_parser::DiffParser},
    one_line::parser::{Commit, Program},
};

pub trait MatchFormat {
    fn print(&self) -> String;
}

pub struct Matcher {
    program: Program,
    search_string: String,
}

pub struct MatcherOutput{
    search_string: String,
    commit_matches: Vec<CommitMatcher>,
    total_matches: usize,
}

type ChannelData = (CommitMatcher, usize);
// 
// impl Matcher {
//     pub fn new(program: Program, search_string: String) -> Self {
//         Self {
//             program,
//             search_string: search_string.to_string(),
//         }
//     }
// 
//     pub fn run(&mut self) -> MatcherOutput {
//         let mut commit_matches: Vec<CommitMatcher> = Vec::new();
//         let mut total_matches: usize = 0;
//         let (tx, rx) = sync::mpsc::channel::<ChannelData>();
//         
//         for commit in self.program.0.iter() { // TODO: change this into .iter()
//             let sender = tx.clone();
//             let search_string = self.search_string.clone();
//             thread::spawn(|| {
//                 thread_runner(commit, search_string, sender);
//             });
//         }
//         return MatcherOutput {
//             commit_matches,
//             total_matches,
//             search_string: String::from("")
//         };
//     }
// }

pub fn do_the_matching(program: Program, search_string: String) -> MatcherOutput {
        let mut commit_matches: Vec<CommitMatcher> = Vec::new();
        let mut total_matches: usize = 0;
        let (tx, rx) = sync::mpsc::channel::<ChannelData>();
        
        let mut messages: usize = 0;
        for commit in program.0.into_iter() { // TODO: change this into .iter()
            messages+= 1;
            let sender = tx.clone();
            let search_string = search_string.clone();
            thread::spawn(|| {
                thread_runner(commit, search_string, sender);
            });
        }

        for _ in 0..messages {
            match rx.recv() {
                Ok((commit, num_matches)) => {
                    commit_matches.push(commit);
                    total_matches += num_matches
                }
                Err(e) => panic!("{}", e)
            }

        }
        return MatcherOutput {
            commit_matches,
            total_matches,
            search_string
        };
}

fn thread_runner(commit: Commit, search_string: String, tx: Sender<ChannelData>) {
    let commit_match = CommitMatcher::find_matches(commit, search_string);
    let num_matches = commit_match.total_matches;
    _ = tx.send((commit_match, num_matches));
}

pub struct CommitMatcher {
    hash: String,
    file_matches: Vec<FileMatches>,
    total_matches: usize,
}

#[derive(Debug)]
struct FileMatches {
    file_header: String,
    content: String,
    matched_lines: usize,
}

impl MatchFormat for MatcherOutput {
    fn print(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Searched for: {}, total matches: {}\n",
            self.search_string, self.total_matches
        ));
        self.commit_matches
            .iter()
            .for_each(|commit_match| out.push_str(&commit_match.print()));
        out.trim().to_string()
    }
}

impl MatchFormat for CommitMatcher {
    fn print(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("for commit hash: {}\n", self.hash));
        out.push_str(&format!("commit matches: {}\n", self.total_matches));
        out.push_str("\n");
        self.file_matches
            .iter()
            .for_each(|file_match| out.push_str(&file_match.print()));
        out.push_str("\n");
        out
    }
}

impl MatchFormat for FileMatches {
    fn print(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("files: {}\n", self.file_header));
        out.push_str(&format!("file matches: {}\n", self.matched_lines));
        out.push_str(&self.content);
        out.push_str("\n");
        out
    }
}

impl CommitMatcher {
    fn find_matches(commit: Commit, search_string: String) -> Self {
        let diff = std::process::Command::new("git")
            .args(["diff", &commit.hash])
            .output()
            .expect(&format!("failed diff for commit {}", &commit.hash));

        let str_diff = std::str::from_utf8(&diff.stdout).expect("couldn't read file");

        let diff_l = DiffLexer::new(str_diff.as_bytes().to_vec());
        let mut diff_p = DiffParser::new(diff_l);
        let diff_program = diff_p.parse_program();

        let mut matches: Vec<FileMatches> = Vec::new();
        let mut total_matches: usize = 0;
        for statement in diff_program.statements.iter() {
            let mut out = String::new();
            let mut matched_lines: usize = 0;
            for chunk in statement.chunks.iter() {
                if chunk.content.len() == 0 {
                    continue;
                }
                for c in chunk.content.iter() {
                    if c.line_data.contains(&search_string) {
                        out.push_str(&format!("{}\n", c.line_data));
                        matched_lines += 1;
                    }
                }
            }
            if matched_lines > 0 {
                matches.push(FileMatches {
                    file_header: format!("diff: {} {}", statement.a_file, statement.b_file),
                    content: out,
                    matched_lines,
                });
                total_matches += matched_lines;
            }
        }

        CommitMatcher {
            hash: commit.hash.clone(),
            file_matches: matches,
            total_matches,
        }
    }
}
