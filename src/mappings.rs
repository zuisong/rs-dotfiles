use anyhow::{Context, Result, bail};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type Mappings = HashMap<String, Vec<PathBuf>>;

#[cfg(windows)]
const PLATFORM: &str = "windows";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "darwin";
#[cfg(target_os = "linux")]
const PLATFORM: &str = "linux";
#[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
const PLATFORM: &str = "unknown";

const UNIX_LIKE_PLATFORM: &str = "unixlike";

fn is_unix_like(platform: &str) -> bool {
    platform == "linux" || platform == "darwin"
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MappingValue {
    Single(String),
    Multiple(Vec<String>),
}

fn get_default_mappings() -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut defaults = HashMap::new();

    let mut windows = HashMap::new();
    windows.insert(".gvimrc".to_string(), vec!["~/vimfiles/gvimrc".to_string()]);
    windows.insert(".vim".to_string(), vec!["~/vimfiles".to_string()]);
    windows.insert(".vimrc".to_string(), vec!["~/vimfiles/vimrc".to_string()]);
    defaults.insert("windows".to_string(), windows);

    let mut unix_like = HashMap::new();
    let common = vec![
        (".agignore", "~/.agignore"),
        (".bash_login", "~/.bash_login"),
        (".bash_profile", "~/.bash_profile"),
        (".bashrc", "~/.bashrc"),
        (".emacs.d", "~/.emacs.d"),
        (".emacs.el", "~/.emacs.d/init.el"),
        (".eslintrc", "~/.eslintrc"),
        (".eslintrc.json", "~/.eslintrc.json"),
        (".eslintrc.yml", "~/.eslintrc.yml"),
        (".gvimrc", "~/.gvimrc"),
        (".npmrc", "~/.npmrc"),
        (".profile", "~/.profile"),
        (".pryrc", "~/.pryrc"),
        (".pylintrc", "~/.pylintrc"),
        (".tmux.conf", "~/.tmux.conf"),
        (".vim", "~/.vim"),
        (".vimrc", "~/.vimrc"),
        (".zlogin", "~/.zlogin"),
        (".zprofile", "~/.zprofile"),
        (".zshenv", "~/.zshenv"),
        (".zshrc", "~/.zshrc"),
        ("agignore", "~/.agignore"),
        ("bash_login", "~/.bash_login"),
        ("bash_profile", "~/.bash_profile"),
        ("bashrc", "~/.bashrc"),
        ("emacs.d", "~/.emacs.d"),
        ("emacs.el", "~/.emacs.d/init.el"),
        ("eslintrc", "~/.eslintrc"),
        ("eslintrc.json", "~/.eslintrc.json"),
        ("eslintrc.yml", "~/.eslintrc.yml"),
        ("gvimrc", "~/.gvimrc"),
        ("npmrc", "~/.npmrc"),
        ("profile", "~/.profile"),
        ("pryrc", "~/.pryrc"),
        ("pylintrc", "~/.pylintrc"),
        ("tmux.conf", "~/.tmux.conf"),
        ("vim", "~/.vim"),
        ("vimrc", "~/.vimrc"),
        ("zlogin", "~/.zlogin"),
        ("zprofile", "~/.zprofile"),
        ("zshenv", "~/.zshenv"),
        ("zshrc", "~/.zshrc"),
        ("init.el", "~/.emacs.d/init.el"),
        ("peco", "~/.config/peco"),
    ];
    for (k, v) in common {
        unix_like.insert(k.to_string(), vec![v.to_string()]);
    }
    defaults.insert(UNIX_LIKE_PLATFORM.to_string(), unix_like);

    let mut linux = HashMap::new();
    linux.insert(".Xmodmap".to_string(), vec!["~/.Xmodmap".to_string()]);
    linux.insert(".Xresources".to_string(), vec!["~/.Xresources".to_string()]);
    linux.insert("Xmodmap".to_string(), vec!["~/.Xmodmap".to_string()]);
    linux.insert("Xresources".to_string(), vec!["~/.Xresources".to_string()]);
    linux.insert("rc.lua".to_string(), vec!["~/.config/rc.lua".to_string()]);
    defaults.insert("linux".to_string(), linux);

    let mut darwin = HashMap::new();
    darwin.insert(".htoprc".to_string(), vec!["~/.htoprc".to_string()]);
    darwin.insert("htoprc".to_string(), vec!["~/.htoprc".to_string()]);
    defaults.insert("darwin".to_string(), darwin);

    defaults
}

pub fn get_mappings(config_dir: &Path) -> Result<Mappings> {
    let mut mappings = Mappings::new();
    let defaults = get_default_mappings();

    if is_unix_like(PLATFORM)
        && let Some(m) = defaults.get(UNIX_LIKE_PLATFORM)
    {
        merge_json_to_mappings(&mut mappings, m)?;
    }
    if let Some(m) = defaults.get(PLATFORM) {
        merge_json_to_mappings(&mut mappings, m)?;
    }

    merge_file_to_mappings(&mut mappings, &config_dir.join("mappings.json"))?;

    if is_unix_like(PLATFORM) {
        merge_file_to_mappings(
            &mut mappings,
            &config_dir.join(format!("mappings_{}.json", UNIX_LIKE_PLATFORM)),
        )?;
    }
    merge_file_to_mappings(
        &mut mappings,
        &config_dir.join(format!("mappings_{}.json", PLATFORM)),
    )?;

    Ok(mappings)
}

fn merge_json_to_mappings(dest: &mut Mappings, src: &HashMap<String, Vec<String>>) -> Result<()> {
    for (k, vs) in src {
        if k.is_empty() {
            bail!("empty key cannot be included");
        }
        let mut paths = Vec::new();
        for v in vs {
            if v.is_empty() {
                continue;
            }
            if !v.starts_with('~') && !v.starts_with('/') {
                bail!(
                    "value of mappings must be an absolute path like '/foo/.bar' or '~/.foo': {}",
                    v
                );
            }
            paths.push(crate::util::expand_tilde(v)?);
        }
        dest.insert(k.clone(), paths);
    }
    Ok(())
}

fn merge_file_to_mappings(dest: &mut Mappings, path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(path)
        .context(format!("Failed to read mapping file: {}", path.display()))?;
    let json: HashMap<String, MappingValue> = serde_json::from_str(&content)?;

    let mut src = HashMap::new();
    for (k, v) in json {
        let vs = match v {
            MappingValue::Single(s) => vec![s],
            MappingValue::Multiple(v) => v,
        };
        src.insert(k, vs);
    }

    merge_json_to_mappings(dest, &src)
}

pub fn create_link(from: &Path, to: &Path, dry: bool) -> Result<bool> {
    if !from.exists() {
        return Ok(false);
    }

    if to.exists() {
        // In Go implementation, it just prints and returns true
        println!("Exist: '{}' -> '{}'", from.display(), to.display());
        return Ok(true);
    }

    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }

    println!(
        "Link:  '{}' -> '{}'",
        from.display().to_string().cyan(),
        to.display()
    );

    if dry {
        return Ok(true);
    }

    if from.is_dir() {
        symlink::symlink_dir(from, to).context(format!(
            "Failed to create directory symlink from {} to {}",
            from.display(),
            to.display()
        ))?;
    } else {
        symlink::symlink_file(from, to).context(format!(
            "Failed to create file symlink from {} to {}",
            from.display(),
            to.display()
        ))?;
    }

    Ok(true)
}

pub fn get_link_source(repo: &Path, to: &Path) -> Result<Option<PathBuf>> {
    let metadata = match fs::symlink_metadata(to) {
        Ok(m) => m,
        Err(_) => return Ok(None),
    };

    if !metadata.file_type().is_symlink() {
        return Ok(None);
    }

    let source = fs::read_link(to)?;
    if source.starts_with(repo) {
        Ok(Some(source))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_default_mappings() {
        let defaults = get_default_mappings();
        assert!(defaults.contains_key("unixlike"));
        assert!(
            defaults.contains_key("darwin")
                || defaults.contains_key("linux")
                || defaults.contains_key("windows")
        );
    }

    #[test]
    fn test_merge_json_to_mappings_single() {
        let mut mappings = Mappings::new();
        let mut src = HashMap::new();
        src.insert(".vimrc".to_string(), vec!["~/.vimrc".to_string()]);
        merge_json_to_mappings(&mut mappings, &src).unwrap();
        assert_eq!(mappings.get(".vimrc").unwrap().len(), 1);
    }

    #[test]
    fn test_merge_json_to_mappings_multiple() {
        let mut mappings = Mappings::new();
        let mut src = HashMap::new();
        src.insert(
            "bashrc".to_string(),
            vec!["~/.bashrc".to_string(), "~/.bash_profile".to_string()],
        );
        merge_json_to_mappings(&mut mappings, &src).unwrap();
        assert_eq!(mappings.get("bashrc").unwrap().len(), 2);
    }

    #[test]
    fn test_merge_json_to_mappings_invalid() {
        let mut mappings = Mappings::new();
        let mut src = HashMap::new();
        src.insert("invalid".to_string(), vec!["not_absolute".to_string()]);
        let result = merge_json_to_mappings(&mut mappings, &src);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_link_source_not_exist() {
        let root = tempdir().unwrap();
        let from = root.path().join("source");
        let to = root.path().join("dest");
        let result = create_link(&from, &to, false).unwrap();
        assert!(!result); // False means skipped because source not exists
        assert!(!to.exists());
    }

    #[test]
    fn test_create_link_already_exists() {
        let root = tempdir().unwrap();
        let from = root.path().join("source");
        let to = root.path().join("dest");
        fs::write(&from, "src").unwrap();
        fs::write(&to, "dest").unwrap();
        let result = create_link(&from, &to, false).unwrap();
        assert!(result); // True means skipped but success (already exists)
        assert!(to.is_file()); // It's still a file, not a link
    }
}
