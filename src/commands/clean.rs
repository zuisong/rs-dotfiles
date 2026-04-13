use crate::mappings::{get_link_source, get_mappings};
use crate::repository::absolute_path_to_repo;
use anyhow::Result;

pub fn execute(repo_input: Option<String>) -> Result<()> {
    let repo = absolute_path_to_repo(repo_input)?;
    let maps = get_mappings(&repo.join(".dotfiles"))?;

    let mut removed = false;
    for tos in maps.values() {
        for to in tos {
            if let Some(src) = get_link_source(&repo, to)? {
                if to.is_dir() {
                    symlink::remove_symlink_dir(to)?;
                } else {
                    symlink::remove_symlink_file(to)?;
                }
                println!("Unlink: '{}' -> '{}'", src.display(), to.display());
                removed = true;
            }
        }
    }

    if !removed {
        println!("No symlink was removed (dotfiles: '{}').", repo.display());
    }

    Ok(())
}
