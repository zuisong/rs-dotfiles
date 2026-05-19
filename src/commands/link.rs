use crate::mappings::{create_link, get_mappings};
use anyhow::{Result, bail};
use std::path::PathBuf;

pub fn execute(repo: PathBuf, specified: Vec<String>, dry: bool) -> Result<()> {
    let maps = get_mappings(&repo.join(".dotfiles"))?;

    let mut created = false;
    if specified.is_empty() {
        for (f, tos) in &maps {
            let from = repo.join(f);
            for to in tos {
                if create_link(&from, to, dry)? {
                    created = true;
                }
            }
        }
    } else {
        for f in specified {
            if let Some(tos) = maps.get(&f) {
                let from = repo.join(&f);
                for to in tos {
                    if create_link(&from, to, dry)? {
                        created = true;
                    }
                }
            }
        }
    }

    if !created {
        bail!(
            "Nothing was linked. '{}' was specified as dotfiles repository. Please check it",
            repo.display()
        );
    }

    Ok(())
}
