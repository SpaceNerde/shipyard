use std::fs;

use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use saphyr::{LoadableYamlNode, Yaml, YamlEmitter};
use tera::Tera;

fn get_metadata(data: &mut String) -> Option<String> {
    if !data.starts_with("---") {
        return None;
    }

    let start = data.find("---").unwrap_or(0);
    data.replace_range(start..3, "");
    let end = data.find("---").unwrap_or(data.len());

    let metadata = data[start..end].trim().to_owned();
    Some(metadata)
}

fn main() {
    let mut file = fs::read_to_string("test/test_file.md").unwrap();
    let template_file = fs::read_to_string("test/test_template.html").unwrap();

    // Parse Markdown to data
    // Get the metadata
    let mut metadata_buffer = get_metadata(&mut file.clone()).unwrap();

    let yaml = Yaml::load_from_str(&metadata_buffer).unwrap();

    let title = yaml[0].as_mapping().unwrap().get_key_value(
        &Yaml::Value(saphyr::Scalar::String(std::borrow::Cow::Borrowed("title")))
    ).unwrap().1.as_str().unwrap();

    println!("{:?}", title);

    // Get the rest of the HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let parser = Parser::new_ext(file.trim(), options);
  
    let mut html_buffer = String::new();
    html::push_html(&mut html_buffer, parser);


    // Insert Data into template and save into file
    let tera = Tera::new("test/**/*").unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("title", title);
    ctx.insert("body_html", &html_buffer);

    let rendered = tera.render("test_template.html", &ctx).unwrap();
    fs::write("test/test_output.html", rendered).unwrap();
}
