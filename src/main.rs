use std::fs;

use pulldown_cmark::{Options, Parser, html};
use saphyr::{LoadableYamlNode, Yaml};
use tera::Tera;

#[derive(Debug, Clone)]
struct Site<'yaml> {
    output_dir: String,
    template_dir: String,
    posts_dir: String,
    html: String,
    yaml: Vec<Yaml<'yaml>>,
    tera: Tera,
}

impl<'yaml> Site<'yaml> {
    fn new() -> Self {
        Site {
            output_dir: "./output".to_string(),
            template_dir: "./templates/**/*.html".to_string(),
            posts_dir: "./posts".to_string(),
            html: String::new(),
            yaml: vec![],
            tera: Tera::default(),
        }
    }

    fn parse_markdown(&mut self) {
        let file = fs::read_to_string(format!("{}/test.md", self.posts_dir)).unwrap();

        // Parse Markdown to data
        // Get the metadata
        let metadata_buffer = self.get_metadata(&mut file.clone()).unwrap();

        self.yaml = Yaml::load_from_str(&metadata_buffer).unwrap();

        // Get the rest of the HTML
        let mut options = Options::empty();
        options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

        let parser = Parser::new_ext(file.trim(), options);

        html::push_html(&mut self.html, parser);
    }

    fn index(&mut self) {
        let title = self.yaml[0]
            .as_mapping()
            .unwrap()
            .get_key_value(&Yaml::Value(saphyr::Scalar::String(
                std::borrow::Cow::Borrowed("title"),
            )))
            .unwrap()
            .1
            .as_str()
            .unwrap();
        let mut ctx = tera::Context::new();

        ctx.insert("title", title);
        ctx.insert("body_html", &self.html);

        let rendered = self.tera.render("index.html", &ctx).unwrap();
        fs::write(format!("{}/index.html", self.output_dir), rendered).unwrap();
    }

    fn about(&mut self) {
        let mut ctx = tera::Context::new();

        ctx.insert("title", "about");

        let rendered = self.tera.render("about.html", &ctx).unwrap();
        fs::write(format!("{}/about.html", self.output_dir), rendered).unwrap();
    }

    fn generate(&mut self) {
        // Insert Data into template and save into file
        self.tera = match Tera::new(&self.template_dir) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        self.index();
        self.about();
    }

    fn get_metadata(&self, data: &mut String) -> Option<String> {
        if !data.starts_with("---") {
            return None;
        }

        let start = data.find("---").unwrap_or(0);
        data.replace_range(start..3, "");
        let end = data.find("---").unwrap_or(data.len());

        let metadata = data[start..end].trim().to_owned();
        Some(metadata)
    }
}

fn main() {
    let mut site = Site::new();
    site.parse_markdown();
    site.generate();
}
