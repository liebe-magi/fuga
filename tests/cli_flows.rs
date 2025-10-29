use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(not(target_os = "windows"))]
use std::fs::read_link;

fn configure_command(cmd: &mut Command, config_dir: &TempDir) {
    cmd.env("XDG_CONFIG_HOME", config_dir.path());
    cmd.env("FUGA_DISABLE_EMOJI", "1");

    #[cfg(target_os = "windows")]
    {
        cmd.env("APPDATA", config_dir.path());
        cmd.env("LOCALAPPDATA", config_dir.path());
    }
}

fn fuga_command(config_dir: &TempDir) -> Command {
    let binary = assert_cmd::cargo::cargo_bin!("fuga");
    let mut cmd = Command::new(binary);
    configure_command(&mut cmd, config_dir);
    cmd
}

fn mark_paths(config_dir: &TempDir, paths: &[&Path]) {
    let mut cmd = fuga_command(config_dir);
    cmd.arg("mark");
    for path in paths {
        cmd.arg(path.to_str().unwrap());
    }
    cmd.assert().success();
}

#[test]
fn mark_missing_path_fails_with_error() {
    let config_dir = TempDir::new().unwrap();
    let missing_dir = TempDir::new().unwrap();
    let missing = missing_dir.child("does-not-exist");

    let mut cmd = fuga_command(&config_dir);
    cmd.args(["mark", missing.path().to_str().unwrap()]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn mark_add_and_list_targets() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let file_a = workspace.child("file_a.txt");
    let file_b = workspace.child("file_b.txt");
    let file_c = workspace.child("file_c.txt");
    file_a.write_str("a").unwrap();
    file_b.write_str("b").unwrap();
    file_c.write_str("c").unwrap();

    mark_paths(&config_dir, &[file_a.path(), file_b.path()]);

    let mut add_cmd = fuga_command(&config_dir);
    add_cmd.arg("mark").arg("--add");
    add_cmd.arg(file_b.path().to_str().unwrap());
    add_cmd.arg(file_c.path().to_str().unwrap());
    add_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(file_c.path().to_str().unwrap()));

    let mut list_cmd = fuga_command(&config_dir);
    list_cmd.args(["mark", "--list"]);
    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(file_a.path().to_str().unwrap()))
        .stdout(predicate::str::contains(file_c.path().to_str().unwrap()));
}

#[test]
fn copy_copies_multiple_targets_into_current_directory() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let source_one = workspace.child("one.txt");
    let source_two = workspace.child("two.log");
    source_one.write_str("one").unwrap();
    source_two.write_str("two").unwrap();

    mark_paths(&config_dir, &[source_one.path(), source_two.path()]);

    let copy_dest = workspace.child("copy_dest");
    copy_dest.create_dir_all().unwrap();

    let mut copy_cmd = fuga_command(&config_dir);
    copy_cmd.current_dir(copy_dest.path());
    copy_cmd.arg("copy");
    copy_cmd.assert().success();

    copy_dest
        .child("one.txt")
        .assert(predicate::path::exists().and(predicate::path::is_file()));
    copy_dest
        .child("two.log")
        .assert(predicate::path::exists().and(predicate::path::is_file()));
}

#[test]
fn copy_requires_marked_targets() {
    let config_dir = TempDir::new().unwrap();

    let mut copy_cmd = fuga_command(&config_dir);
    copy_cmd.arg("copy");
    copy_cmd
        .assert()
        .failure()
        .stderr(predicate::str::contains("No targets marked."));
}

#[test]
fn move_resets_mark_list_after_success() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let source_one = workspace.child("move_source_a.txt");
    let source_two = workspace.child("move_source_b.txt");
    source_one.write_str("alpha").unwrap();
    source_two.write_str("beta").unwrap();

    mark_paths(&config_dir, &[source_one.path(), source_two.path()]);

    let move_dest = workspace.child("move_dest");
    move_dest.create_dir_all().unwrap();

    let mut move_cmd = fuga_command(&config_dir);
    move_cmd.args(["move", move_dest.path().to_str().unwrap()]);
    move_cmd.assert().success();

    move_dest
        .child("move_source_a.txt")
        .assert(predicate::path::exists().and(predicate::path::is_file()));
    move_dest
        .child("move_source_b.txt")
        .assert(predicate::path::exists().and(predicate::path::is_file()));
    assert!(!source_one.path().exists());
    assert!(!source_two.path().exists());

    let mut list_cmd = fuga_command(&config_dir);
    list_cmd.args(["mark", "--list"]);
    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("No targets marked."));
}

#[test]
fn multi_target_to_single_destination_is_error() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let file_one = workspace.child("multi_a.txt");
    let file_two = workspace.child("multi_b.txt");
    file_one.write_str("a").unwrap();
    file_two.write_str("b").unwrap();

    mark_paths(&config_dir, &[file_one.path(), file_two.path()]);

    let mut move_cmd = fuga_command(&config_dir);
    move_cmd.args(["move", "single.txt"]);
    move_cmd.assert().failure().stderr(predicate::str::contains(
        "Cannot move multiple items to a single file path.",
    ));
}

#[cfg(unix)]
#[test]
fn mark_reports_permission_denied() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let dir = workspace.child("restricted");
    dir.create_dir_all().unwrap();
    let file = dir.child("secret.txt");
    file.write_str("classified").unwrap();

    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o000)).unwrap();

    let target_path = file.path().to_path_buf();
    let mut cmd = fuga_command(&config_dir);
    cmd.args(["mark", target_path.to_str().unwrap()]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Permission denied"));

    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn link_command_creates_symlinks_for_multiple_targets() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let source_one = workspace.child("link_one.txt");
    let source_two = workspace.child("link_two.txt");
    source_one.write_str("one").unwrap();
    source_two.write_str("two").unwrap();

    mark_paths(&config_dir, &[source_one.path(), source_two.path()]);

    let link_dest = workspace.child("links");
    link_dest.create_dir_all().unwrap();

    let mut link_cmd = fuga_command(&config_dir);
    link_cmd.args(["link", link_dest.path().to_str().unwrap()]);
    link_cmd.assert().success();

    let link_one = link_dest.path().join("link_one.txt");
    let link_two = link_dest.path().join("link_two.txt");
    assert!(link_one.exists());
    assert!(link_two.exists());
    assert_eq!(read_link(&link_one).unwrap(), source_one.path());
    assert_eq!(read_link(&link_two).unwrap(), source_two.path());
}

#[test]
fn copy_detects_duplicate_destination() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let file = workspace.child("document.txt");
    file.write_str("content").unwrap();

    mark_paths(&config_dir, &[file.path()]);

    let mut copy_cmd = fuga_command(&config_dir);
    copy_cmd.current_dir(file.path().parent().unwrap());
    copy_cmd.arg("copy");
    copy_cmd.assert().failure().stderr(predicate::str::contains(
        "Source and destination are the same",
    ));
}

#[test]
fn legacy_single_target_is_migrated_to_targets() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let legacy_file = workspace.child("legacy.txt");
    legacy_file.write_str("legacy").unwrap();
    let legacy_path = legacy_file.path().canonicalize().unwrap();

    let config_root = config_dir.path().join("fuga");
    fs::create_dir_all(&config_root).unwrap();
    fs::write(
        config_root.join("fuga.toml"),
        format!("[data]\ntarget = \"{}\"\n", legacy_path.to_str().unwrap()),
    )
    .unwrap();

    let mut list_cmd = fuga_command(&config_dir);
    list_cmd.args(["mark", "--list"]);
    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(legacy_path.to_str().unwrap()));

    let stored = fs::read_to_string(config_root.join("fuga.toml")).unwrap();
    assert!(stored.contains("targets"));
    assert!(!stored
        .lines()
        .any(|line| line.trim_start().starts_with("target =")));
}
