use std::{
    sync::{self, Arc},
    thread,
};

use crate::{
    diff::{diff_lexer::DiffLexer, diff_parser::DiffParser},
    formatter::{Color, FormatBuilder, Styles},
    one_line::parser::{Commit, Program}, Options,
};

pub struct MatcherOutput {
    search_string: String,
    commit_matches: Vec<CommitMatcher>,
    total_matches: usize,
}

pub struct CommitMatcher {
    hash: String,
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
    let messages: usize = program.0.len();
    let search_arc = Arc::new(options.clone());

    for commit in program.0.into_iter() {
        let tx = tx.clone();
        let options_arc = search_arc.clone();

        thread::spawn(move || {
            let commit_match = CommitMatcher::find_matches(commit, options_arc);
            let num_matches = commit_match.total_matches;
            _ = tx.send((commit_match, num_matches));
        });
    }

    let mut commit_matches: Vec<CommitMatcher> = Vec::new();
    let mut total_matches: usize = 0;
    let mut message_num: usize = 0;

    for (commit, num_matches) in rx {
        message_num += 1;
        if num_matches > 0 { // TODO: add an option whether to show 0 result commits
            commit_matches.push(commit);
            total_matches += num_matches
        }

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
    EmptyDiff
}

impl CommitMatcher {
    fn find_matches(commit: Commit, options: Arc<Options>) -> Self {
        let mut diff_args = vec!["diff", &commit.hash];
        // get additional context from git diff if needed
        let context_needed = options.before_context.max(options.after_context);
        let with_context = &format!("-U{}", context_needed);
        if context_needed > 0 {
            diff_args.push(with_context);
        }

        let diff = std::process::Command::new("git")
            .args(diff_args)
            .output()
            .expect(&format!("failed diff for commit {}", &commit.hash));

        let str_diff = std::str::from_utf8(&diff.stdout).expect("couldn't read file");

        // early exit if there is no content from the diff
        if str_diff.len() == 0 {
            return CommitMatcher {
                hash: commit.hash.clone(),
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
            let mut out = String::new();
            let mut matched_lines: usize = 0;
            for chunk in statement.chunks.iter() {
                if chunk.content.len() == 0 {
                    continue;
                }

                // record which lines were captured
                // also get the before/afetr context if requested
                for c in 0..chunk.content.len() {
                    if chunk.content[c].line_data.contains(options.search_string.as_str()) {
                        out.push_str(&format!("{}\n", chunk.content[c].line_data));
                        matched_lines += 1;
                    }
                }
            }
            if matched_lines > 0 {
                matches.push(FileMatches {
                    file_a: statement.a_file,
                    file_b: statement.b_file,
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

pub trait MatchFormat {
    fn print(&self) -> String;
}

impl MatchFormat for MatcherOutput {
    fn print(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} \"{}\"\n{} {}\n\n",
            FormatBuilder::new("Searched for:")
                .color(Color::Green)
                .add_style(Styles::Bold)
                .build(),
            self.search_string,
            FormatBuilder::new("Total Matches:")
                .color(Color::Green)
                .add_style(Styles::Bold)
                .build(),
            self.total_matches
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
        out.push_str(&format!(
            "{} {}\n",
            FormatBuilder::new("For commit hash:")
                .color(Color::Cyan)
                .build(),
            FormatBuilder::new(&self.hash)
                .color(Color::Cyan)
                .add_style(Styles::Bold)
                .build()
        ));
        out.push_str(&format!(
            "{} {}\n",
            FormatBuilder::new("Commit matches:")
                .color(Color::Cyan)
                .build(),
            FormatBuilder::new(&self.total_matches.to_string())
                .color(Color::Cyan)
                .add_style(Styles::Bold)
                .build()
        ));
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
        out.push_str(&format!(
            "{}\n",
            FormatBuilder::new(&format!("diff: {} {}", &self.file_a, &self.file_b))
                .add_style(Styles::Italic)
                .color(Color::Green)
                .build()
        ));
        out.push_str(&format!(
            "{} {}\n",
            FormatBuilder::new("File matches:")
                .color(Color::Green)
                .build(),
            FormatBuilder::new(&self.matched_lines.to_string())
                .add_style(Styles::Bold)
                .color(Color::Green)
                .build()
        ));
        out.push_str(&self.content);
        out.push_str("\n");
        out
    }
}
