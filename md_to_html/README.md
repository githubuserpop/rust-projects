# Markdown to Html File Converter

This command-line application is written in Rust and converts Markdown files to HTML.

## Usage

md_to_html <input_file>

Replace `<input_file>` with the path to the Markdown file you want to convert.

## Code Overview

### Main Function

The `main` function is the entry point of the application. It performs the following steps:

1. **Parse Command Line Arguments**: It expects exactly one argument, the path to the input file. If the number of arguments is not correct, it prints a usage message and exits.
    
2. **Read Input File**: It reads the content of the input file into a string. If there's an error reading the file, it prints an error message and exits.
    
3. **Parse Markdown Content**: It parses the Markdown content of the input file using the `pulldown_cmark` library. The parsed content is converted into HTML and stored in a string.
    
4. **Save HTML Output**: It saves the HTML content to an output file. The output file is in the same directory as the input file and has the same name, but with a `.html` extension. If there's an error writing to the output file, it prints an error message.
    

### get_output_file_path Function

The `get_output_file_path` function takes the path to the input file as a string and returns a `PathBuf` representing the path to the output file. It does this by changing the extension of the input file path to `.html`.

## Dependencies

This application uses the following crates:

- `std::env`: For parsing command line arguments.
- `std::fs`: For reading from and writing to files.
- `pulldown_cmark`: For parsing Markdown content and converting it to HTML.
- `std::path::PathBuf`: For manipulating file paths.
