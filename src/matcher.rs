use std::{
    sync::{self, Arc},
    thread,
};

use crate::{
    diff::{diff_lexer::DiffLexer, diff_parser::DiffParser},
    formatter::{Color, StyleBuilder, Styles},
    pretty_medium::parser::{Commit, Program},
    Options,
};

pub struct MatcherOutput {
    search_string: String,
    commit_matches: Vec<CommitMatcher>,
    total_matches: usize,
}

pub struct CommitMatcher {
    commit: Commit,
    previous_hash: String,
    file_matches: Vec<FileMatches>,
    total_matches: usize,
}

#[derive(Debug)]
struct FileMatches {
    file_a: String,
    file_b: String,
    content: String,
    matched_lines: usize,
}

type ChannelData = (CommitMatcher, usize);

pub fn do_the_matching(program: Program, options: Options) -> MatcherOutput {
    let (tx, rx) = sync::mpsc::channel::<ChannelData>();
    let messages: usize = program.0.len() - 1;
    let options_arc = Arc::new(options.clone());

    for c_idx in 1..program.0.len() {
        let tx = tx.clone();
        let options_arc = options_arc.clone();

        let commit = program.0[c_idx].clone();
        let previous_hash = program.0[c_idx - 1].hash.clone();

        thread::spawn(move || {
            let commit_match = CommitMatcher::find_matches(commit, previous_hash, options_arc);
            let num_matches = commit_match.total_matches;
            _ = tx.send((commit_match, num_matches));
        });
    }
    let mut commit_matches: Vec<CommitMatcher> = Vec::new();
    let mut total_matches: usize = 0;
    let mut message_num: usize = 0;

    for (commit, num_matches) in rx {
        commit_matches.push(commit);

        message_num += 1;
        total_matches += num_matches;

        if message_num >= messages {
            break;
        }
    }

    return MatcherOutput {
        commit_matches,
        total_matches,
        search_string: options.search_string,
    };
}

#[allow(dead_code)]
enum CommitMatcherErrors {
    DiffError(Vec<String>),
    EmptyDiff,
}

impl CommitMatcher {
    fn find_matches(commit: Commit, previous_commit: String, options: Arc<Options>) -> Self {
        let mut diff_args = vec!["diff", &commit.hash, &previous_commit];
        // get additional context from git diff if needed
        let context_needed = options.before_context.max(options.after_context);
        let with_context = &format!("-U{}", context_needed);
        if context_needed > 0 {
            diff_args.push(with_context);
        }

        let mut com = std::process::Command::new("git");
        com.args(diff_args);
        if let Some(t) = &options.target_dir {
            com.current_dir(t);
        }

        let diff = com.output().expect(&format!(
            "failed diff for commits {}, {previous_commit}",
            commit.hash
        ));
        let str_diff = std::str::from_utf8(&diff.stdout).expect("couldn't read file");

        // early exit if there is no content from the diff
        if str_diff.len() == 0 {
            return CommitMatcher {
                commit,
                previous_hash: previous_commit.to_string(),
                file_matches: Vec::new(),
                total_matches: 0,
            };
        }

        let diff_l = DiffLexer::new(str_diff.as_bytes().to_vec());
        let mut diff_p = DiffParser::new(diff_l);
        let diff_program = diff_p.parse_program();

        let mut matches: Vec<FileMatches> = Vec::new();
        let mut total_matches: usize = 0;

        for statement in diff_program.statements.into_iter() {
            let (content, matched_lines) = statement.fmt(options.clone());

            if matched_lines > 0 {
                matches.push(FileMatches {
                    file_a: statement.a_file,
                    file_b: statement.b_file,
                    content,
                    matched_lines,
                });
                total_matches += matched_lines;
            }
        }
        CommitMatcher {
            commit,
            previous_hash: previous_commit.to_string(),
            file_matches: matches,
            total_matches,
        }
    }
}

pub trait MatchFormat {
    fn print(&self, options: Options) -> String;
    fn simple_print(&self, options: Options) -> String;
}

impl MatchFormat for MatcherOutput {
    fn print(&self, options: Options) -> String {
        let cyan_bold = StyleBuilder::new(&options.color)
            .add_style(Styles::Color(Color::Cyan))
            .add_style(Styles::Bold);
        let mut out = String::new();
        out.push_str(&format!(
            "{} \"{}\"\n{} {}\n\n",
            cyan_bold.build("Searched For:"),
            self.search_string,
            cyan_bold.build("Total Matches:"),
            self.total_matches
        ));

        self.commit_matches.iter().for_each(|commit_match| {
            if !options.show_empty && commit_match.total_matches == 0 {
                return;
            }
            out.push_str(&commit_match.print(options.clone()))
        });
        out.trim().to_string()
    }

    fn simple_print(&self, options: Options) -> String {
        let mut out = String::new();
        self.commit_matches.iter().for_each(|commit_match| {
            if !options.show_empty && commit_match.total_matches == 0 {
                return;
            }
            out.push_str(&commit_match.simple_print(options.clone()))
        });
        out.trim().to_string()
    }
}

impl MatchFormat for CommitMatcher {
    fn print(&self, options: Options) -> String {
        let cyan = StyleBuilder::new(&options.color).add_style(Styles::Color(Color::Cyan));
        let cyan_bold = cyan.clone().add_style(Styles::Bold);

        let mut out = String::new();
        out.push_str(&format!(
            "{} {} {}\n",
            cyan.build("git diff"),
            cyan_bold.build(&self.commit.hash),
            cyan_bold.build(&self.previous_hash),
        ));
        out.push_str(&format!(
            "{} {}\n",
            cyan.build("message:"),
            cyan_bold.build(&self.commit.message),
        ));
        out.push_str(&format!(
            "{} {}\n",
            cyan.build("date:"),
            cyan_bold.build(&self.commit.date),
        ));
        out.push_str(&format!(
            "{} {}\n",
            cyan.build("Commit matches:"),
            cyan_bold.build(&self.total_matches.to_string()),
        ));
        out.push_str("\n");
        self.file_matches.iter().for_each(|file_match| {
            if !options.show_empty && file_match.matched_lines == 0 {
                return;
            }
            out.push_str(&file_match.print(options.clone()))
        });
        out.push_str("\n");
        out
    }

    fn simple_print(&self, options: Options) -> String {
        let cyan = StyleBuilder::new(&options.color).add_style(Styles::Color(Color::Cyan));
        let cyan_bold = cyan.clone().add_style(Styles::Bold);

        let mut out = String::new();
        out.push_str(&format!(
            "{} {} {}\n",
            cyan.build("git diff"),
            cyan_bold.build(&self.commit.hash),
            cyan_bold.build(&self.previous_hash),
        ));
        self.file_matches.iter().for_each(|file_match| {
            if !options.show_empty && file_match.matched_lines == 0 {
                return;
            }
            out.push_str(&file_match.simple_print(options.clone()))
        });
        out.push_str("\n");
        out
    }
}

impl MatchFormat for FileMatches {
    fn print(&self, options: Options) -> String {
        let cyan = StyleBuilder::new(&options.color).add_style(Styles::Color(Color::Cyan));
        let cyan_it = cyan.clone().add_style(Styles::Italic);
        let cyan_bold = cyan.clone().add_style(Styles::Bold);

        let mut out = String::new();
        if !options.skip_file_print {
            // print file details
            out.push_str(&format!(
                "{}\n",
                cyan_it.build(&format!("file diff: {} {}", &self.file_a, &self.file_b)),
            ));
            out.push_str(&format!(
                "{} {}\n",
                cyan.build("File matches:"),
                cyan_bold.build(&self.matched_lines.to_string()),
            ));
        }
        out.push_str(&self.content);
        out.push_str("\n");
        out
    }

    fn simple_print(&self, options: Options) -> String {
        let cyan_it = StyleBuilder::new(&options.color)
            .add_style(Styles::Color(Color::Cyan))
            .add_style(Styles::Italic);

        let mut out = String::new();
        if !options.skip_file_print {
            // print file details
            out.push_str(&format!(
                "{}\n",
                cyan_it.build(&format!("file diff: {} {}", &self.file_a, &self.file_b)),
            ));
        }
        out.push_str(&self.content);
        out.push_str("\n");
        out
    }
}
