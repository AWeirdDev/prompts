use std::process::exit;

use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use indicatif::ProgressBar;
use inquire::Select;

use pyo3::prelude::*;

mod connection;
mod macros;
mod md_parser;

use crate::connection::get_prompt;
use crate::md_parser::parse_md;

#[derive(Parser)]
#[command(version = "0.1.0", about = "aip is an AI prompt manager.")]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Adds a prompt to the current workspace.
    Add(AddCommand),
    Preview(PreviewCommand),
}

#[derive(Args)]
struct AddCommand {
    /// Name of the prompt. Offline path or online ID.
    name: String,
    #[arg(short, long, default_value = "false")]
    fill: bool,
}

#[derive(Args)]
struct PreviewCommand {
    /// Name of the prompt. Offline path or online ID.
    name: String,
}

#[pyfunction]
fn main(args: Vec<String>) {
    let cli = CLI::parse_from(args);

    ctrlc::set_handler(|| println!("^C")).ok();

    match cli.command {
        Command::Add(AddCommand { name, fill }) => {
            let content = match std::fs::read_to_string(name.to_owned()) {
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
            let result = parse_md(content);

            match result {
                Some(metadata) => {
                    println!(
                        "\n{}: {}\n{}: {}\n{}: {}\n",
                        "name".bold(),
                        metadata
                            .name
                            .unwrap_or(tostring!("None"))
                            .green()
                            .bold()
                            .underline(),
                        "description".bold().dimmed(),
                        metadata.description.unwrap_or(tostring!("None")).dimmed(),
                        "author".bold().dimmed(),
                        metadata.author.unwrap_or(tostring!("Unknown")).dimmed(),
                    );

                    let select_result = Select::new(
                        "How do you want to add this to your project?",
                        vec!["🐍 Python-typed", "📄 Plain text"],
                    )
                    .prompt();

                    match select_result {
                        Ok(result) => {
                            if result == "🐍 Python-typed" {
                                println!("{}", "→ Got it. I'll make it Pythonic!\n".dimmed());
                            } else if result == "📄 Plain text" {
                                println!("{}", "→ Sure! I'll leave it as it is.\n".dimmed());
                            }
                        }
                        Err(e) => println!("{}: {}", "error".bold().red(), e),
                    }

                    if fill {
                        println!(
                            "📲 {}: Since you passed {} flag, let's fill the values manually.",
                            "Filling".bold().cyan(),
                            "--fill".bold()
                        );
                    }
                }
                None => println!(
                    "{}: failed to parse metadata for {}",
                    "error".red().bold(),
                    name.cyan()
                ),
            }
        }
        Command::Preview(PreviewCommand { name }) => {
            let bar = ProgressBar::new_spinner()
                .with_message(format!("Fetching with {}", "ureq".bold().cyan()));
            bar.enable_steady_tick(std::time::Duration::from_millis(50));

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

            bar.finish();
            println!("{content}");
        }
    }
}

#[pymodule]
fn aip(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
