use std::fs;

use pulldown_cmark::{html, Options, Parser};
use tera::Tera;

fn main() {
    let file = fs::read_to_string("test/test_file.md").unwrap();
    let template_file = fs::read_to_string("test/test_template.html").unwrap();
    
    // Parse Markdown to data
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut parser = Parser::new_ext(file.trim(), options);

    println!("{:?}", &parser);

    let mut html_buffer = String::new();
    html::push_html(&mut html_buffer, parser);

    print!("{}", html_buffer);

    // Insert Data into template and save into file
    let mut tera = Tera::new("test/**/*").unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", "TEST");
    ctx.insert("body_html", &html_buffer);

    let rendered = tera.render("test_template.html", &ctx).unwrap();
    fs::write("test/test_output.html", rendered).unwrap();
}
