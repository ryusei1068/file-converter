use std::env::{self, args};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::process::exit;

fn read_file(path: &String) -> String {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open file. {} {:?}", path, e);
            exit(1)
        }
    };

    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    match reader.read_to_string(&mut buf) {
        Err(e) => {
            eprintln!("Failed to read. {:?}", e);
            exit(1)
        }
        _ => {}
    };

    buf
}

fn md_parse(contents: &str) -> String {
    let parser = pulldown_cmark::Parser::new(contents);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}

fn write_html(path: &String, html: &String) {
    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create a file. {:?}", e);
            exit(1)
        }
    };
    let mut buffer = BufWriter::new(file);
    match buffer.write_all(html.as_bytes()) {
        Err(e) => {
            eprintln!("Failed to write. {:?}", e);
            exit(1)
        }
        _ => {}
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let output_path = &args[2];
    let buf = read_file(input_path);

    let html = md_parse(&buf);
    write_html(output_path, &html);
}
