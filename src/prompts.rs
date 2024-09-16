use std::fs;

use anyhow::Result;

use crate::{connection::gh_get_prompt, inquiries::inquire_config, md_parser::Metadata};

/// Gets a prompt.
/// If the name does not exist in the current directory, fetches from github.
pub fn get_prompt(name: String) -> Result<String> {
    let binding = std::fs::exists(name.to_owned() + ".md")?;
    if binding {
        let text = fs::read_to_string(name + ".md")?;
        Ok(text)
    } else {
        let prompt = gh_get_prompt(name)?;
        Ok(prompt)
    }
}

/// Adds a prompt.
/// Edits config.json if needed.
pub fn add_prompt(fname: String, metadata: Metadata, _content: String) -> Result<()> {
    if matches!(metadata.name, None) {
        return Err(anyhow::anyhow!("No name provided"));
    }

    let mut config = inquire_config()?;
    if !config.dependencies.contains(&fname) {
        config.dependencies.push(fname);
        config.record()?;
    }

    if !fs::exists("prompts")? {
        fs::create_dir("prompts")?;
    }
    Ok(())
}
