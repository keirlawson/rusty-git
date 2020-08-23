use rustygit::Repository;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str;
use tempfile;

#[test]
fn test_init() {
    let dir = tempfile::tempdir().unwrap();

    Repository::init(&dir).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(str::from_utf8(&output.stdout).unwrap().trim(), "true");
}

#[test]
fn test_add_single() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!["somefile"]).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("somefile"));
}

#[test]
fn test_add_multiple() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    repo.add(vec!["somefile", "anotherfile"]).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("somefile"));
    assert!(str::from_utf8(&output.stdout)
        .unwrap()
        .contains("anotherfile"));
}

#[test]
fn test_commit_all() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!["somefile"]).unwrap();

    repo.commit_all("some commit message").unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["log"])
        .output()
        .unwrap();

    assert!(str::from_utf8(&output.stdout)
        .unwrap()
        .contains("some commit message"));
}

#[test]
fn test_remove_uncommitted_single() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!["somefile"]).unwrap();
    let result = repo.remove(vec!["somefile"], false);

    assert!(result.is_err());
}

#[test]
fn test_remove_uncommitted_single_force() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!["somefile"]).unwrap();
    repo.remove(vec!["somefile"], true).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(!str::from_utf8(&output.stdout).unwrap().contains("somefile"));
}

#[test]
fn test_remove_committed_single() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!["somefile"]).unwrap();
    repo.commit_all("some msg").unwrap();
    repo.remove(vec!["somefile"], false).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(!str::from_utf8(&output.stdout).unwrap().contains("somefile"));
}

#[test]
fn test_remove_commited_multiple() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    repo.add(vec!["somefile", "anotherfile"]).unwrap();
    repo.commit_all("some msg").unwrap();

    repo.remove(vec!["somefile", "anotherfile"], false).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(!str::from_utf8(&output.stdout).unwrap().contains("somefile"));
    assert!(!str::from_utf8(&output.stdout)
        .unwrap()
        .contains("anotherfile"));
}

#[test]
fn test_remove_uncommited_multiple() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    repo.add(vec!["somefile", "anotherfile"]).unwrap();
    let result = repo.remove(vec!["somefile", "anotherfile"], false);

    assert!(result.is_err());
}

#[test]
fn test_remove_uncommited_multiple_force() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    repo.add(vec!["somefile", "anotherfile"]).unwrap();
    repo.remove(vec!["somefile", "anotherfile"], true).unwrap();

    let output = Command::new("git")
        .current_dir(&dir)
        .args(&["ls-files"])
        .output()
        .unwrap();

    assert!(!str::from_utf8(&output.stdout).unwrap().contains("somefile"));
    assert!(!str::from_utf8(&output.stdout)
        .unwrap()
        .contains("anotherfile"));
}

#[test]
fn test_list_added() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    repo.add(vec!["somefile", "anotherfile"]).unwrap();

    let output = repo.list_added().unwrap();

    assert!(output.contains(&String::from("somefile")));
    assert!(output.contains(&String::from("anotherfile")));
}

#[test]
fn test_list_untracked() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    let output = repo.list_untracked().unwrap();

    assert!(output.contains(&String::from("somefile")));
    assert!(output.contains(&String::from("anotherfile")));
}

#[test]
fn test_list_modified() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    let mut file1 = File::create(dir.as_ref().join("somefile")).unwrap();
    let mut file2 = File::create(dir.as_ref().join("anotherfile")).unwrap();
    repo.add(vec!["somefile", "anotherfile"]).unwrap();
    repo.commit_all("some msg").unwrap();

    file1.write(b"Hello there!").unwrap();
    file2.write(b"General Kenobi").unwrap();

    let output = repo.list_modified().unwrap();

    assert!(output.contains(&String::from("somefile")));
    assert!(output.contains(&String::from("anotherfile")));
}

#[test]
fn test_list_tracked() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();

    let output = repo.list_tracked().unwrap();

    assert!(!output.contains(&String::from("somefile")));
    assert!(!output.contains(&String::from("anotherfile")));

    repo.add(vec!["somefile"]).unwrap();

    let output = repo.list_tracked().unwrap();
    assert!(output.contains(&String::from("somefile")));
    assert!(!output.contains(&String::from("anotherfile")));

    repo.add(vec!["anotherfile"]).unwrap();

    let output = repo.list_tracked().unwrap();

    assert!(output.contains(&String::from("somefile")));
    assert!(output.contains(&String::from("anotherfile")));

    repo.commit_all("some_msg").unwrap();

    let output = repo.list_tracked().unwrap();

    assert!(output.contains(&String::from("somefile")));
    assert!(output.contains(&String::from("anotherfile")));
}

#[test]
fn test_get_hash() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    fs::write(&dir.as_ref().join("somefile"), "Some content").unwrap();
    repo.add(vec!["somefile"]).unwrap();
    repo.commit_all("Commit 1").unwrap();

    let hash1_short = repo.get_hash(true).unwrap();
    let hash1_long = repo.get_hash(false).unwrap();
    assert!(hash1_long.starts_with(&hash1_short),);

    fs::write(&dir.as_ref().join("anotherfile"), "Some content").unwrap();
    repo.add(vec!["anotherfile"]).unwrap();
    repo.commit_all("Commit 2").unwrap();

    let hash2_short = repo.get_hash(true).unwrap();
    let hash2_long = repo.get_hash(false).unwrap();
    assert!(hash2_long.starts_with(&hash2_short));

    assert_ne!(hash1_short, hash2_short);
}
