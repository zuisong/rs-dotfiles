use anyhow::{Context, Result};
use directories::UserDirs;
use std::path::{Path, PathBuf};

pub fn expand_tilde<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    if !path_str.starts_with('~') {
        return Ok(path.to_path_buf());
    }

    let user_dirs = UserDirs::new().context("Could not determine user home directory")?;
    let home_dir = user_dirs.home_dir();

    if path_str == "~" {
        return Ok(home_dir.to_path_buf());
    }

    if let Some(stripped) = path_str.strip_prefix("~/") {
        return Ok(home_dir.join(stripped));
    }

    // For other cases like ~user, we just return the path for now as original Go code
    // mostly handles ~/ or absolute paths.
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expand_tilde_no_tilde() {
        let path = "/usr/bin/git";
        assert_eq!(expand_tilde(path).unwrap(), PathBuf::from(path));
    }

    #[test]
    fn test_expand_tilde_with_home() {
        let path = "~/.vimrc";
        let expanded = expand_tilde(path).unwrap();
        assert!(expanded.is_absolute());
        assert!(expanded.to_string_lossy().ends_with(".vimrc"));
        assert!(!expanded.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_expand_tilde_only_tilde() {
        let expanded = expand_tilde("~").unwrap();
        assert!(expanded.is_absolute());
    }
}
