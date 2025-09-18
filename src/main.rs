use std::fs;

use pulldown_cmark::{Options, Parser, html};
use saphyr::{LoadableYamlNode, Yaml};
use tera::{Context, Tera};

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

    fn get_context(&self) -> Context {
        let mut ctx = Context::new();

        // yeah it aint a beauty but it will do for now
        let _ = self.yaml[0].clone().into_mapping().map(|yaml| {
            for key in yaml.keys() {
                ctx.insert(
                    key.as_str().unwrap(),
                    &yaml.get_key_value(key).unwrap().0.as_str(),
                );
            }
        });

        ctx.insert("body_html", &self.html);

        ctx
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

        self.tera.get_template_names().for_each(|template| {
            let rendered = self.tera.render(template, &self.get_context()).unwrap();
            fs::write(format!("{}/{}", self.output_dir, template), rendered).unwrap();
        });
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
