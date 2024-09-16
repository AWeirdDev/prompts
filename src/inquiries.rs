use std::{fs, process::exit};

use anyhow::Result;
use colored::Colorize;
use inquire::Select;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::tostring;

#[derive(Serialize, Deserialize)]
pub enum Binding {
    #[serde(rename = "pythonic")]
    Pythonic,
    #[serde(rename = "plain")]
    Plain,
}

impl Binding {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pythonic => "pythonic",
            Self::Plain => "plain",
        }
    }
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub binding: Binding,
    pub dependencies: Vec<String>,
}

impl Config {
    /// Writes the config to config.json
    pub fn record(&self) -> Result<()> {
        let mut v = serde_json::to_value(self)?;
        v.as_object_mut()
            .unwrap()
            .insert(tostring!("$schema"), json!("./schema.json"));

        fs::write(".aip/config.json", format!("{:#}", v))?;
        Ok(())
    }
}

pub fn inquire_config() -> Result<Config> {
    let dir_exists = fs::exists(".aip/")?;
    let config_exists = fs::exists(".aip/config.json")?;

    if !dir_exists || !config_exists {
        if !dir_exists {
            fs::create_dir(".aip")?;
            fs::write(".aip/.gitignore", "*")?;
            fs::write(
                ".aip/schema.json",
                json!({
                    "type": "object",
                    "properties": {
                        "binding": {
                            "type": "string",
                            "description": "Bindings for the prompt.",
                            "enum": [
                                "pythonic",
                                "plain"
                            ]
                        },
                        "dependencies": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Dependencies for the prompt."
                        }
                    }
                })
                .to_string(),
            )?;
        }

        let select_result = Select::new(
            "How do you want to add this to your project?",
            vec!["🐍 Python", "📄 Plain text"],
        )
        .prompt();

        let binding = match select_result {
            Ok(result) => {
                if result == "🐍 Python" {
                    println!("{}", "→ Got it. I'll make it Pythonic!\n".dimmed());
                    "pythonic"
                } else if result == "📄 Plain text" {
                    println!("{}", "→ Sure! I'll leave it as it is.\n".dimmed());
                    "plain"
                } else {
                    exit(-1);
                }
            }
            Err(e) => {
                println!("{}: {:?}", "error".bold().red(), e);
                exit(-1);
            }
        };

        let config = json!({
            "schema": "./schema.json",
            "binding": binding,
            "dependencies": []
        });

        fs::write(".aip/config.json", format!("{:#}", config))?;

        println!("\n{}", "File summary".underline());
        println!("{}", ".aip/".dimmed());
        if !dir_exists {
            println!("{} {}", "+".bold().green(), ".gitignore");
            println!("{} {}", "+".bold().green(), "schema.json");
        }
        println!("{} {}", "+".bold().green(), "config.json");
        println!();

        return Ok(serde_json::from_value::<Config>(config)?);
    }

    Ok(serde_json::from_str::<Config>(
        fs::read_to_string(".aip/config.json")?.as_str(),
    )?)
}
