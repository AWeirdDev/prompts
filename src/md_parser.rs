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
            if !doc["name"].is_badvalue() {
                match metadata.add(
                    tostring!("name"),
                    doc["name"]
                        .to_owned()
                        .or(Yaml::String(tostring!("None")))
                        .as_str()
                        .unwrap()
                        .to_string(),
                ) {
                    Ok(_) => (),
                    Err(e) => println!("{}: {}", "error".bold().red(), e),
                }
            }

            if !doc["author"].is_badvalue() {
                match metadata.add(
                    tostring!("author"),
                    doc["author"]
                        .to_owned()
                        .or(Yaml::String(tostring!("Unknown")))
                        .as_str()
                        .unwrap()
                        .to_string(),
                ) {
                    Ok(_) => (),
                    Err(e) => println!("{}: {}", "error".bold().red(), e),
                }
            }

            if !doc["description"].is_badvalue() {
                match metadata.add(
                    tostring!("description"),
                    doc["description"]
                        .to_owned()
                        .or(Yaml::String(tostring!("None")))
                        .as_str()
                        .unwrap()
                        .to_string(),
                ) {
                    Ok(_) => (),
                    Err(e) => println!("{}: {}", "error".bold().red(), e),
                }
            }
        }

        Some(metadata)
    } else {
        None
    }
}
