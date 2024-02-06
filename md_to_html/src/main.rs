use std::env;
use std::fs;
use pulldown_cmark::{html, Options, Parser};
use std::path::PathBuf;
fn main() {
    // Get the input file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: md_to_html <input_file>");
        return;
    }
    let input_file = &args[1];

    // Read the input file
    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };

    // Parse the Markdown content
    let parser = Parser::new_ext(&input_content, Options::empty());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Save the HTML output to a file in the same directory as the input file
    let output_file = get_output_file_path(input_file);
    match fs::write(&output_file, html_output) {
        Ok(_) => println!("HTML output saved to {}", output_file.display()),
        Err(err) => eprintln!("Error saving HTML output: {}", err),
    }
}

fn get_output_file_path(input_file: &str) -> PathBuf {
    let mut output_file = PathBuf::from(input_file);
    output_file.set_extension("html");
    output_file
}
