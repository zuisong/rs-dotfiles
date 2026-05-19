use crate::mappings::{get_link_source, get_mappings};
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(repo: PathBuf) -> Result<()> {
    let maps = get_mappings(&repo.join(".dotfiles"))?;

    let mut links = Vec::new();
    for tos in maps.values() {
        for to in tos {
            if let Some(src) = get_link_source(&repo, to)? {
                links.push((src, to.clone()));
            }
        }
    }

    links.sort_by(|a, b| a.0.cmp(&b.0));
    links.dedup();

    for (src, dst) in &links {
        println!("'{}' -> '{}'", src.display(), dst.display());
    }

    if links.is_empty() {
        println!("No link was found (dotfiles: {})", repo.display());
    }

    Ok(())
}
