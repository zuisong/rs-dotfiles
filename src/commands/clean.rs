use crate::mappings::{get_link_source, get_mappings};
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(repo: PathBuf) -> Result<()> {
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
