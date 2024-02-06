use std::process::Command;
use std::fs::{File, OpenOptions};
use std::collections::VecDeque;
use std::path::Path;
use rustyline::Editor;
use rustyline::highlight::Highlighter;
use syntect::easy::HighlightLines;
use syntect::util::as_24_bit_terminal_escaped;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::{SyntaxSet, ParseState, SyntaxReference}; // Add SyntaxReference import

struct ShellHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    parse_state: ParseState,
}

impl Highlighter for ShellHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let syntax = self.syntax_set.find_syntax_plain_text().first_line_match.unwrap();
        let mut h = HighlightLines::new(&syntax, &self.theme_set.themes["base16-ocean.dark"]); // Remove & before syntax
        let ranges: Vec<(Style, &str)> = h.highlight(line, &self.syntax_set); // Pass SyntaxSet argument
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        std::borrow::Cow::Owned(escaped)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        false
    }
}

fn main() {
    let mut history: VecDeque<String> = VecDeque::new();
    let mut child_processes: Vec<std::process::Child> = Vec::new();

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_plain_text().first_line_match.unwrap();
    let parse_state = ParseState::new(&syntax); // Remove & before syntax
    let h = ShellHighlighter { syntax_set, theme_set, parse_state };
    let mut rl = Editor::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                let input = input.trim();

                if input == "exit" {
                    // Terminate all child processes before exiting
                    for mut child in child_processes {
                        child.kill().unwrap();
                    }
                    break;
                }

                if input.is_empty() {
                    continue;
                }

                if input == "history" {
                    for cmd in &history {
                        println!("{}", cmd);
                    }
                    continue;
                }

                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let args: Vec<&str> = parts.collect();

                let mut cmd = Command::new(command);
                cmd.args(args);

                if let Some(last_cmd) = history.back() {
                    if last_cmd.contains(" > ") {
                        let output_file = last_cmd.split(" > ").last().unwrap();
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(output_file)
                            .unwrap();
                        cmd.stdout(file);
                    } else if last_cmd.contains(" >> ") {
                        let output_file = last_cmd.split(" >> ").last().unwrap();
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(output_file)
                            .unwrap();
                        cmd.stdout(file);
                    }
                }

                if let Some(last_cmd) = history.back() {
                    if last_cmd.contains(" < ") {
                        let input_file = last_cmd.split(" < ").last().unwrap();
                        let file = File::open(input_file).unwrap();
                        cmd.stdin(file);
                    }
                }

                let child = cmd.spawn();

                match child {
                    Ok(child) => {
                        child_processes.push(child);
                    }
                    Err(err) => eprintln!("Failed to execute command: {}", err),
                }

                history.push_back(input.to_string());

                if history.len() > 10 {
                    history.pop_front();
                }
            }
            Err(_) => break,
        }
    }
}