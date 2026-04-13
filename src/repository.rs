use anyhow::{Result, bail};
use std::env;
use std::path::PathBuf;

pub fn absolute_path_to_repo(repo: Option<String>) -> Result<PathBuf> {
    let repo_path = match repo {
        Some(r) if !r.is_empty() => r,
        _ => env::var("DOTFILES_REPO_PATH").unwrap_or_else(|_| {
            eprintln!("No repository was specified nor $DOTFILES_REPO_PATH was not set. Assuming current repository is a dotfiles repository.\n");
            ".".to_string()
        })
    };

    let p = crate::util::expand_tilde(&repo_path)?;
    if !p.exists() || !p.is_dir() {
        bail!(
            "'{}' is not a directory. Please specify your dotfiles directory",
            p.display()
        );
    }
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_absolute_path_to_repo_parameter() {
        let root = tempdir().unwrap();
        let repo_dir = root.path().join("repo");
        fs::create_dir(&repo_dir).unwrap();
        let result = absolute_path_to_repo(Some(repo_dir.to_str().unwrap().to_string())).unwrap();
        assert_eq!(
            fs::canonicalize(result).unwrap(),
            fs::canonicalize(repo_dir).unwrap()
        );
    }

    #[test]
    fn test_absolute_path_to_repo_env() {
        let root = tempdir().unwrap();
        let repo_dir = root.path().join("env_repo");
        fs::create_dir(&repo_dir).unwrap();

        let old_env = std::env::var("DOTFILES_REPO_PATH").ok();
        unsafe {
            std::env::set_var("DOTFILES_REPO_PATH", repo_dir.to_str().unwrap());
        }

        let result = absolute_path_to_repo(None).unwrap();
        assert_eq!(
            fs::canonicalize(result).unwrap(),
            fs::canonicalize(repo_dir).unwrap()
        );

        unsafe {
            if let Some(v) = old_env {
                std::env::set_var("DOTFILES_REPO_PATH", v);
            } else {
                std::env::remove_var("DOTFILES_REPO_PATH");
            }
        }
    }

    #[test]
    fn test_absolute_path_to_repo_not_exist() {
        let result = absolute_path_to_repo(Some("/non/existent/path".to_string()));
        assert!(result.is_err());
    }
}
