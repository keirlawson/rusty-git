use rustygit::Repository;
use tempfile;
use std::process::Command;
use std::str;
use std::fs::File;

#[test]
fn test_init() {
    let dir = tempfile::tempdir().unwrap();

    Repository::init(&dir).unwrap();

    let output = Command::new("git").current_dir(&dir).args(&["rev-parse", "--is-inside-work-tree"]).output().unwrap();

    assert!(output.status.success());
    assert_eq!(str::from_utf8(&output.stdout).unwrap().trim(), "true");
}

#[test]
fn test_add_single() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!("somefile")).unwrap();

    let output = Command::new("git").current_dir(&dir).args(&["ls-files"]).output().unwrap();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("somefile"));
}

#[test]
fn test_add_multiple() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();
    File::create(dir.as_ref().join("anotherfile")).unwrap();


    repo.add(vec!("somefile", "anotherfile")).unwrap();


    let output = Command::new("git").current_dir(&dir).args(&["ls-files"]).output().unwrap();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("somefile"));
    assert!(str::from_utf8(&output.stdout).unwrap().contains("anotherfile"));
}

#[test]
fn test_commit_all() {
    let dir = tempfile::tempdir().unwrap();

    let repo = Repository::init(&dir).unwrap();

    File::create(dir.as_ref().join("somefile")).unwrap();

    repo.add(vec!("somefile")).unwrap();

    repo.commit_all("some commit message").unwrap();
    
    let output = Command::new("git").current_dir(&dir).args(&["log"]).output().unwrap();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("some commit message"));
}