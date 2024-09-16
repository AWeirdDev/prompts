use std::process::exit;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use indicatif::ProgressBar;

use pyo3::prelude::*;

mod connection;
mod inquiries;
mod macros;
mod md_parser;
mod prompts;

use crate::inquiries::inquire_config;
use crate::md_parser::parse_md;
use crate::prompts::add_prompt;
use crate::prompts::get_prompt as _get_prompt;

#[derive(Parser)]
#[command(version = "0.1.0", about = "🦍 aip is an AI prompt manager.")]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Adds a prompt to the current workspace.
    Add(AddCommand),
    /// Previews a prompt.
    Preview(PreviewCommand),
}

#[derive(Args)]
struct AddCommand {
    /// Name of the prompts. Offline path or online ID.
    names: Vec<String>,
    #[arg(short, long, default_value = "false")]
    fill: bool,
}

#[derive(Args)]
struct PreviewCommand {
    /// Name of the prompt. Offline path or online ID.
    name: String,
}

fn get_prompt(name: String) -> Result<String> {
    let bar = ProgressBar::new_spinner()
        .with_message(format!("Getting {}", name.to_owned().bold().cyan()));
    bar.enable_steady_tick(std::time::Duration::from_millis(50));

    let content = _get_prompt(name.to_owned())?;
    bar.finish_with_message(format!("{} {}", "+".bold().green(), name.to_owned()));
    Ok(content)
}

#[pyfunction]
fn main(args: Vec<String>) {
    let cli = CLI::parse_from(args);

    ctrlc::set_handler(|| println!("^C")).ok();

    match cli.command {
        Command::Add(AddCommand { names, fill }) => {
            let _config = match inquire_config() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}: {:?}", "error".bold().red(), e);
                    exit(-1);
                }
            };

            for name in names {
                let content = match get_prompt(name.to_owned()) {
                    Ok(c) => c,
                    Err(e) => {
                        println!(
                            "{}: failed to load: {:?} {}",
                            "error".bold().red(),
                            e,
                            format!("({})", name).dimmed()
                        );
                        exit(-1);
                    }
                };
                println!();

                let result = parse_md(content.to_owned());

                match result {
                    Some(m) => {
                        if fill {
                            println!(
                                "📲 {}: Since you passed {} flag, let's fill the values manually.",
                                "Filling".bold().cyan(),
                                "--fill".bold()
                            );
                        }

                        add_prompt(name, m, content).unwrap();
                    }
                    None => {}
                }
            }
        }
        Command::Preview(PreviewCommand { name }) => {
            let content = match get_prompt(name.to_owned()) {
                Ok(t) => t,
                Err(e) => {
                    println!(
                        "{}: failed to fetch prompt {:?}: {:?}",
                        "error".bold().red(),
                        name,
                        e
                    );
                    exit(-1);
                }
            };
            println!("\n{content}");
        }
    }
}

#[pymodule]
fn aip(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
