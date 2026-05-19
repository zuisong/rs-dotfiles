use anyhow::Result;
use clap::{CommandFactory, Parser};
use rs_dotfiles::cli::{Cli, Commands};
use rs_dotfiles::commands;
use std::io;
use std::process;

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Link { repo, dry, files } => {
            let path = rs_dotfiles::repository::absolute_path_to_repo(repo)?;
            commands::link::execute(path, files, dry)
        }
        Commands::List { repo } => {
            let path = rs_dotfiles::repository::absolute_path_to_repo(repo)?;
            commands::list::execute(path)
        }
        Commands::Clean { repo } => {
            let path = rs_dotfiles::repository::absolute_path_to_repo(repo)?;
            commands::clean::execute(path)
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "dotfiles", &mut io::stdout());
            Ok(())
        }
        Commands::Version => {
            println!("dotfiles version {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("Error: {}", err);
        process::exit(113);
    }
}
