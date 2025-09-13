use markdown::to_html;
use std::fs;

fn main() {
    let file = fs::read_to_string("test/test_file.md").unwrap();

    println!("{}", to_html(file.trim()));
}
