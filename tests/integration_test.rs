use rs_dotfiles::commands::{clean, link, list};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_link_and_clean() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    // Setup dotfiles repo
    let repo_dir = root_path.join("my_dotfiles");
    let config_dir = repo_dir.join(".dotfiles");
    fs::create_dir_all(&config_dir).unwrap();

    // Create a source file
    let source_file = repo_dir.join("test_config.conf");
    fs::write(&source_file, "some config").unwrap();

    // Create a destination path
    let dest_file = root_path.join("final_config.conf");

    // Create mappings.json
    let mappings_json = config_dir.join("mappings.json");
    let content = format!(
        r#"{{ "test_config.conf": "{}" }}"#,
        dest_file.to_str().unwrap()
    );
    fs::write(mappings_json, content).unwrap();

    // Run link command
    let repo_input = Some(repo_dir.to_str().unwrap().to_string());
    link::execute(repo_input.clone(), vec![], false).expect("Link command failed");

    // Verify link exists
    assert!(dest_file.exists(), "Link should have been created");
    let metadata = fs::symlink_metadata(&dest_file).unwrap();
    assert!(
        metadata.file_type().is_symlink(),
        "Created file should be a symlink"
    );

    // Run list command (output should be to stdout, but we just check if it doesn't crash)
    list::execute(repo_input.clone()).expect("List command failed");

    // Run clean command
    clean::execute(repo_input).expect("Clean command failed");

    // Verify link is removed
    assert!(!dest_file.exists(), "Link should have been removed");
}

#[test]
fn test_link_dry_run() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    let repo_dir = root_path.join("my_dotfiles");
    let config_dir = repo_dir.join(".dotfiles");
    fs::create_dir_all(&config_dir).unwrap();

    let source_file = repo_dir.join("test_config.conf");
    fs::write(&source_file, "some config").unwrap();

    let dest_file = root_path.join("final_config.conf");

    let mappings_json = config_dir.join("mappings.json");
    let content = format!(
        r#"{{ "test_config.conf": "{}" }}"#,
        dest_file.to_str().unwrap()
    );
    fs::write(mappings_json, content).unwrap();

    // Run link command with dry-run
    let repo_input = Some(repo_dir.to_str().unwrap().to_string());
    link::execute(repo_input, vec![], true).expect("Dry-run Link command failed");

    // Verify link does NOT exist
    assert!(
        !dest_file.exists(),
        "Link should NOT have been created in dry-run mode"
    );
}

#[test]
fn test_link_specific_files() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    let repo_dir = root_path.join("my_dotfiles");
    let config_dir = repo_dir.join(".dotfiles");
    fs::create_dir_all(&config_dir).unwrap();

    let source1 = repo_dir.join("config1.conf");
    let source2 = repo_dir.join("config2.conf");
    fs::write(&source1, "c1").unwrap();
    fs::write(&source2, "c2").unwrap();

    let dest1 = root_path.join("d1.conf");
    let dest2 = root_path.join("d2.conf");

    let mappings_json = config_dir.join("mappings.json");
    let content = format!(
        r#"{{ "config1.conf": "{}", "config2.conf": "{}" }}"#,
        dest1.to_str().unwrap(),
        dest2.to_str().unwrap()
    );
    fs::write(mappings_json, content).unwrap();

    let repo_input = Some(repo_dir.to_str().unwrap().to_string());

    // Only link config1
    link::execute(repo_input, vec!["config1.conf".to_string()], false)
        .expect("Link specific failed");

    assert!(dest1.exists());
    assert!(!dest2.exists());
}

#[test]
fn test_link_nothing_linked_error() {
    let root = tempdir().unwrap();
    let root_path = root.path();

    let repo_dir = root_path.join("empty_repo");
    fs::create_dir_all(repo_dir.join(".dotfiles")).unwrap();

    let repo_input = Some(repo_dir.to_str().unwrap().to_string());
    let result = link::execute(repo_input, vec![], false);

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Nothing was linked")
    );
}
