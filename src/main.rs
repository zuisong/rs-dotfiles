use clap::{CommandFactory, Parser};
use rs_dotfiles::cli::{Cli, Commands};
use rs_dotfiles::commands;
use std::io;
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Link { repo, dry, files } => commands::link::execute(repo, files, dry),
        Commands::List { repo } => commands::list::execute(repo),
        Commands::Clean { repo } => commands::clean::execute(repo),
        Commands::Update { repo } => commands::update::execute(repo),
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "dotfiles", &mut io::stdout());
            Ok(())
        }
        Commands::Version => {
            println!("dotfiles version {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(113);
    }
}
