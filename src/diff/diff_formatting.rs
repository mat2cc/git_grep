use std::sync::Arc;
use crate::{
    formatter::{ColorTrait, StyleTrait},
    Options,
};

use super::diff_ast::{Content, ContentType, Statement};

impl Content {
    pub fn fmt(&self, search_string: &str) -> String {
        let out_line = format!("{}    {}\n", self.c_type.to_string(), self.line_data);
        if self.c_type == ContentType::Neutral {
            return out_line;
        }

        let colorize = |s: &str| match self.c_type {
            ContentType::Add => s.green(),
            ContentType::Remove => s.red(),
            _ => unreachable!(),
        };
        let out_line = out_line
            .split(search_string)
            .map(|x| colorize(x))
            .collect::<Vec<String>>()
            .join(&search_string.bold().cyan());

        return out_line;
    }
}

impl Statement {
    pub fn fmt(&self, options: Arc<Options>) -> (String, usize) {
        let mut out = String::new();
        let mut matched_lines = 0;
        let mut lines = vec![false; self.data.len()];
        for (idx, l) in self.data.iter().enumerate() {
            if l.c_type != ContentType::Neutral && l.line_data.contains(&options.search_string) {
                matched_lines += 1;
                add_context(
                    &mut lines,
                    idx,
                    options.before_context,
                    options.after_context,
                );
            }
        }

        for x in 0..self.data.len() {
            if lines[x] {
                out.push_str(&self.data[x].fmt(&options.search_string));
                // add a spacer when we have reached a break in context
                if x + 1 < self.data.len() && !lines[x + 1] {
                    out.push_str("\n");
                }
            }
        }

        (out, matched_lines)
    }
}

fn add_context(mut lines: &mut Vec<bool>, idx: usize, pre_context: usize, post_context: usize) {
    lines[idx] = true;
    if pre_context > 0 && idx > 0 {
        // make sure we have more pre-context, and we stay in bounds
        add_context(&mut lines, idx - 1, pre_context - 1, post_context);
    }
    if post_context > 0 && idx + 1 < lines.len() {
        // make sure we have more post-context, and we stay in bounds
        add_context(&mut lines, idx + 1, pre_context, post_context - 1);
    }
}
