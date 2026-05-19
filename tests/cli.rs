use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("dotfiles version"));
}

#[test]
fn test_completion() {
    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd.args(["completion", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_dotfiles()"));
}

#[test]
fn test_list_empty_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo_dir = temp.child("repo");
    repo_dir.create_dir_all().unwrap();

    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd.arg("list")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No link was found"));
}

#[test]
fn test_link_dry_run() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo_dir = temp.child("repo");
    repo_dir.create_dir_all().unwrap();

    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    // Providing invalid arg or empty mapping to test failure if no file created
    cmd.arg("link")
        .arg("--dry")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .failure() // because "Nothing was linked" error is returned when maps are empty
        .stderr(predicate::str::contains("Nothing was linked"));
}

#[test]
fn test_clean_no_links() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo_dir = temp.child("repo");
    repo_dir.create_dir_all().unwrap();

    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd.arg("clean")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No symlink was removed"));
}

#[test]
fn test_link_and_list_and_clean() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Create fake repo
    let repo_dir = temp.child("repo");
    repo_dir.create_dir_all().unwrap();

    // Create config dir
    let dotfiles_dir = repo_dir.child(".dotfiles");
    dotfiles_dir.create_dir_all().unwrap();

    // Write dummy files to be linked
    let source_file1 = repo_dir.child("test_config1");
    source_file1.write_str("dummy content").unwrap();
    let source_file2 = repo_dir.child("test_config2");
    source_file2.write_str("dummy content").unwrap();

    // Write mapping.json
    let dest_file1 = temp.child("dest_config1");
    let dest_file2 = temp.child("dest_config2");
    let dest_path1 = dest_file1.path().to_string_lossy().replace("\\", "\\\\");
    let dest_path2 = dest_file2.path().to_string_lossy().replace("\\", "\\\\");

    let mappings_json = dotfiles_dir.child("mappings.json");
    mappings_json
        .write_str(&format!(
            r#"{{"test_config1": "{}", "test_config2": "{}"}}"#,
            dest_path1, dest_path2
        ))
        .unwrap();

    // Run link
    let mut cmd_link = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_link
        .arg("link")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Link:"));

    assert!(dest_file1.path().exists());
    assert!(dest_file2.path().exists());

    // Run list
    let mut cmd_list = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_list
        .arg("list")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test_config1"))
        .stdout(predicate::str::contains("test_config2"));

    // Run clean
    let mut cmd_clean = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_clean
        .arg("clean")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Unlink:"));

    assert!(!dest_file1.path().exists());
    assert!(!dest_file2.path().exists());

    // Link specific file not in mappings
    let mut cmd_link_not_in_map = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_link_not_in_map
        .arg("link")
        .arg(repo_dir.path())
        .arg("not_in_mappings.conf")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Nothing was linked"));
}

#[test]
fn test_link_and_clean_dir() {
    let temp = assert_fs::TempDir::new().unwrap();
    let repo_dir = temp.child("repo");
    repo_dir.create_dir_all().unwrap();
    let dotfiles_dir = repo_dir.child(".dotfiles");
    dotfiles_dir.create_dir_all().unwrap();

    // Create a directory to link
    let source_dir = repo_dir.child("test_dir");
    source_dir.create_dir_all().unwrap();

    let dest_dir = temp.child("dest_dir");
    let dest_path = dest_dir.path().to_string_lossy().replace("\\", "\\\\");

    let mappings_json = dotfiles_dir.child("mappings.json");
    mappings_json
        .write_str(&format!(r#"{{"test_dir": "{}"}}"#, dest_path))
        .unwrap();

    // Run link
    let mut cmd_link = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_link
        .arg("link")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success();

    assert!(dest_dir.path().exists());
    assert!(dest_dir.path().is_dir());

    // Run clean
    let mut cmd_clean = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd_clean
        .arg("clean")
        .env("DOTFILES_REPO_PATH", repo_dir.path())
        .assert()
        .success();

    assert!(!dest_dir.path().exists());
}

#[test]
fn test_link_non_existent_repo() {
    let mut cmd = Command::cargo_bin("rs-dotfiles").unwrap();
    cmd.arg("link")
        .arg("/non/existent/path/for/repo")
        .assert()
        .failure()
        .stderr(predicates::str::contains("is not a directory"));
}
