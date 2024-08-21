use colored::Colorize;
use yaml_rust2::{Yaml, YamlLoader};

use crate::tostring;

pub struct Metadata {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            name: None,
            author: None,
            description: None,
        }
    }

    pub fn add(&mut self, name: String, value: String) -> Result<(), String> {
        match name.as_str() {
            "name" => Ok(self.name = Some(value)),
            "author" => Ok(self.author = Some(value)),
            "description" => Ok(self.description = Some(value)),
            _ => Err(format!("Unknown metadata key: {}", name)),
        }
    }

    pub fn add_from_yaml(&mut self, index: &'static str, doc: &Yaml) -> Result<(), String> {
        if !doc[index].is_badvalue() {
            match self.add(
                index.to_string(),
                doc[index]
                    .to_owned()
                    .or(Yaml::String(tostring!("None")))
                    .as_str()
                    .unwrap()
                    .to_string(),
            ) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}: {}", "error".bold().red(), e)),
            }
        } else {
            Ok(())
        }
    }
}

pub fn parse_md(content: String) -> Option<Metadata> {
    let lines = content
        .split('\n')
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    if lines[0].trim() == "---" {
        let mut i = 1_usize;
        let mut fences = String::new();

        while i < lines.len() {
            if lines[i].trim() == "---" {
                break;
            }
            fences.push_str(&lines[i]);
            i += 1;
        }

        let mut metadata = Metadata::new();
        let docs = YamlLoader::load_from_str(&fences).unwrap();
        for doc in docs {
            let _ = metadata.add_from_yaml("name", &doc);
            let _ = metadata.add_from_yaml("author", &doc);
            let _ = metadata.add_from_yaml("description", &doc);
        }

        Some(metadata)
    } else {
        None
    }
}
