use anyhow::{Result, bail};
use std::env;
use std::path::PathBuf;

pub fn absolute_path_to_repo(repo: Option<String>) -> Result<PathBuf> {
    let env_repo = env::var("DOTFILES_REPO_PATH").ok();
    absolute_path_to_repo_inner(repo, env_repo)
}

fn absolute_path_to_repo_inner(repo: Option<String>, env_repo: Option<String>) -> Result<PathBuf> {
    let repo_path = match repo {
        Some(r) if !r.is_empty() => r,
        _ => env_repo.unwrap_or_else(|| {
            eprintln!("No repository was specified nor $DOTFILES_REPO_PATH was not set. Assuming current repository is a dotfiles repository.\n");
            ".".to_string()
        }),
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
        let result =
            absolute_path_to_repo_inner(Some(repo_dir.to_str().unwrap().to_string()), None)
                .unwrap();
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

        // Simulate old_env was None (not provided)
        // In this case, absolute_path_to_repo_inner falls back to "."
        let result1 = absolute_path_to_repo_inner(None, None).unwrap();
        assert_eq!(
            fs::canonicalize(result1).unwrap(),
            fs::canonicalize(".").unwrap()
        );

        // Simulate old_env was Some
        let result2 =
            absolute_path_to_repo_inner(None, Some(repo_dir.to_str().unwrap().to_string()))
                .unwrap();
        assert_eq!(
            fs::canonicalize(result2).unwrap(),
            fs::canonicalize(&repo_dir).unwrap()
        );
    }

    #[test]
    fn test_absolute_path_to_repo_not_exist() {
        let result = absolute_path_to_repo_inner(Some("/non/existent/path".to_string()), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_absolute_path_to_repo_no_env() {
        let result = absolute_path_to_repo_inner(None, None).unwrap();
        // Since we return None for env var, it should default to "."
        // and resolve to current dir absolute path.
        assert_eq!(
            fs::canonicalize(result).unwrap(),
            fs::canonicalize(".").unwrap()
        );
    }
}
