use crate::repository::absolute_path_to_repo;
use anyhow::{Context, Result};
use std::process::Command;

pub fn execute(repo_input: Option<String>) -> Result<()> {
    let repo = absolute_path_to_repo(repo_input)?;

    let mut cmd = Command::new("git");
    cmd.arg("pull").current_dir(&repo);

    let status = cmd.status().context("Failed to execute git pull")?;
    if !status.success() {
        anyhow::bail!("git pull failed with status: {}", status);
    }

    Ok(())
}
