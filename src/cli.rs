use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dotfiles", about = "A dotfiles symlinks manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Put symlinks to setup your configurations
    Link {
        /// Path to your dotfiles repository. If omitted, $DOTFILES_REPO_PATH is searched.
        #[arg(env = "DOTFILES_REPO_PATH")]
        repo: Option<String>,
        /// Show what happens only
        #[arg(long)]
        dry: bool,
        /// Files to link. If you specify no file, all will be linked.
        files: Vec<String>,
    },
    /// Show a list of symbolic link put by this command
    List {
        #[arg(env = "DOTFILES_REPO_PATH")]
        repo: Option<String>,
    },
    /// Remove all symbolic links put by this command
    Clean {
        #[arg(env = "DOTFILES_REPO_PATH")]
        repo: Option<String>,
    },
    /// Update your dotfiles repository
    Update {
        #[arg(env = "DOTFILES_REPO_PATH")]
        repo: Option<String>,
    },
    /// Generate shell completion script
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Show version
    Version,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
