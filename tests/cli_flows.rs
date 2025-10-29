use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
#[cfg(unix)]
use std::fs;

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

#[test]
fn mark_missing_path_fails_with_error() {
    let config_dir = TempDir::new().unwrap();
    let missing = config_dir.child("does-not-exist");

    let mut cmd = fuga_command(&config_dir);
    cmd.args(["mark", missing.path().to_str().unwrap()]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn copy_and_move_commands_succeed() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let source_dir = workspace.child("source");
    source_dir.create_dir_all().unwrap();
    let source_file = source_dir.child("note.txt");
    source_file.write_str("hello world").unwrap();

    // Mark the source file
    let mut mark_cmd = fuga_command(&config_dir);
    mark_cmd.args(["mark", source_file.path().to_str().unwrap()]);
    mark_cmd.assert().success();

    // Copy into destination directory
    let copy_dest = workspace.child("copy_dest");
    copy_dest.create_dir_all().unwrap();

    let mut copy_cmd = fuga_command(&config_dir);
    copy_cmd.current_dir(copy_dest.path());
    copy_cmd.arg("copy");
    copy_cmd.assert().success();
    copy_dest
        .child("note.txt")
        .assert(predicate::path::exists().and(predicate::path::is_file()));

    // Prepare for move
    let move_source = workspace.child("move_me.txt");
    move_source.write_str("move me").unwrap();

    let mut mark_move_cmd = fuga_command(&config_dir);
    mark_move_cmd.args(["mark", move_source.path().to_str().unwrap()]);
    mark_move_cmd.assert().success();

    let move_dest = workspace.child("move_dest");
    move_dest.create_dir_all().unwrap();

    let mut move_cmd = fuga_command(&config_dir);
    move_cmd.current_dir(move_dest.path());
    move_cmd.arg("move");
    move_cmd.assert().success();
    move_dest
        .child("move_me.txt")
        .assert(predicate::path::exists().and(predicate::path::is_file()));
    assert!(!move_source.path().exists());
}

#[test]
fn copy_detects_duplicate_destination() {
    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let file = workspace.child("document.txt");
    file.write_str("content").unwrap();

    let mut mark_cmd = fuga_command(&config_dir);
    mark_cmd.args(["mark", file.path().to_str().unwrap()]);
    mark_cmd.assert().success();

    let mut copy_cmd = fuga_command(&config_dir);
    copy_cmd.current_dir(file.path().parent().unwrap());
    copy_cmd.arg("copy");
    copy_cmd.assert().failure().stderr(predicate::str::contains(
        "Source and destination are the same",
    ));
}

#[cfg(unix)]
#[test]
fn mark_reports_permission_denied() {
    use std::os::unix::fs::PermissionsExt;

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

    // Restore perms so tempdir cleanup succeeds
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn link_command_creates_symlink() {
    use std::fs::read_link;

    let workspace = TempDir::new().unwrap();
    let config_dir = TempDir::new().unwrap();

    let source = workspace.child("source.txt");
    source.write_str("hello").unwrap();

    let mut mark_cmd = fuga_command(&config_dir);
    mark_cmd.args(["mark", source.path().to_str().unwrap()]);
    mark_cmd.assert().success();

    let link_dest = workspace.child("links");
    link_dest.create_dir_all().unwrap();

    let mut link_cmd = fuga_command(&config_dir);
    link_cmd.current_dir(link_dest.path());
    link_cmd.arg("link");
    link_cmd.assert().success();

    let symlink_path = link_dest.path().join("source.txt");
    assert!(symlink_path.exists());
    assert_eq!(read_link(symlink_path).unwrap(), source.path());
}
