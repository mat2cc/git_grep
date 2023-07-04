use crate::{
    diff::{diff_lexer::DiffLexer, diff_parser::DiffParser},
    one_line::parser::{Commit, Program},
};

pub trait MatchFormat {
    fn print(&self) -> String;
}

pub struct Matcher<'a> {
    commit_matches: Vec<CommitMatcher<'a>>,
    total_matches: usize,
    search_string: &'a str,
}

impl<'a> Matcher<'a> {
    pub fn new(program: &'a Program, search_string: &'a str) -> Self {
        let mut commit_matches: Vec<CommitMatcher> = Vec::new();
        let mut total_matches: usize = 0;
        for commit in program.0.iter() {
            let commit_match = find_commit_matches(commit, search_string);
            total_matches += commit_match.total_matches;
            commit_matches.push(commit_match);
        }
        Self {
            search_string,
            commit_matches,
            total_matches,
        }
    }
}

pub struct CommitMatcher<'a> {
    hash: &'a str,
    file_matches: Vec<FileMatches>,
    total_matches: usize,
}

#[derive(Debug)]
struct FileMatches {
    file_header: String,
    content: String,
    matched_lines: usize,
}

impl<'a> MatchFormat for Matcher<'a> {
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

impl<'a> MatchFormat for CommitMatcher<'a> {
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

fn find_commit_matches<'a>(commit: &'a Commit, search_string: &'a str) -> CommitMatcher<'a> {
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
                if c.line_data.contains(search_string) {
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
        hash: &commit.hash,
        file_matches: matches,
        total_matches,
    }
}
