use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use inquire::Select;

use md_parser::parse_md;

mod md_parser;

#[derive(Parser)]
#[command(version = "0.1.0", about)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Adds a prompt to the current workspace.
    Add(AddCommand),
}

#[derive(Args)]
struct AddCommand {
    /// Name of the prompt.
    name: String,
    #[arg(short, long, default_value = "false")]
    fill: bool,
}

macro_rules! tostring {
    ($($x:tt)*) => {
        format!($($x)*)
    };
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Command::Add(AddCommand { name, fill }) => {
            let result = parse_md(std::fs::read_to_string(name.to_owned()).unwrap());

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
                                println!("Got it. I'll make it Pythonic!\n");
                            } else if result == "📄 Plain text" {
                                println!("Sure! I'll leave it as it is.\n");
                            }
                        }
                        Err(e) => println!("{}: {}", "error".bold().red(), e),
                    }

                    if fill {
                        println!(
                            "Since you passed {} flag, let's fill the values statically.",
                            "--fill".bold()
                        )
                    }
                }
                None => println!(
                    "{}: failed to parse metadata for {}",
                    "error".red().bold(),
                    name.cyan()
                ),
            }
        }
    }
}
