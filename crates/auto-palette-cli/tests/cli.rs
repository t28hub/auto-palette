use assert_cmd::Command;
use predicates::prelude::*;

/// Returns the auto-palette command.
///
/// # Returns
/// The auto-palette command.
#[must_use]
fn auto_palette() -> Command {
    Command::cargo_bin("auto-palette-cli").unwrap()
}

#[test]
fn test_cli() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--algorithm")
        .arg("dbscan")
        .arg("--count")
        .arg("6")
        .arg("--no-resize")
        .assert()
        .stdout(
            predicate::str::contains("#FFFFFF")
                .and(predicate::str::contains("#0081C8"))
                .and(predicate::str::contains("#FCB131"))
                .and(predicate::str::contains("#EE344E"))
                .and(predicate::str::contains("#000000"))
                .and(predicate::str::contains("Extracted 6 swatch(es) in")),
        );
    assert.success();
}

#[test]
fn test_version() {
    let assert = auto_palette()
        .arg("--version")
        .assert()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    assert.success();
}

#[test]
fn test_help() {
    let assert = auto_palette()
        .arg("--help")
        .assert()
        .stdout(predicate::str::contains(env!("CARGO_PKG_DESCRIPTION")));
    assert.success();
}
