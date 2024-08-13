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
        .arg("--output")
        .arg("table")
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
fn test_missing_path() {
    let assert = auto_palette().assert().stderr(predicate::str::contains(
        "🎨 A CLI tool to extract prominent color palettes from images.",
    ));
    assert.failure();
}

#[test]
fn test_invalid_path() {
    let assert = auto_palette()
        .arg("invalid.png")
        .assert()
        .stderr(predicate::str::contains(
            "Failed to open the image file \"invalid.png\"",
        ));
    assert.failure();
}

#[test]
fn test_algorithm() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--algorithm")
        .arg("kmeans")
        .assert();
    assert.success();
}

#[test]
fn test_invalid_algorithm() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--algorithm")
        .arg("unknown")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value 'unknown' for '--algorithm <name>'",
        ));
    assert.failure();
}

#[test]
fn test_theme() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--theme")
        .arg("light")
        .assert();
    assert.success();
}

#[test]
fn test_invalid_theme() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--theme")
        .arg("unknown")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value 'unknown' for '--theme <name>'",
        ));
    assert.failure();
}

#[test]
fn test_invalid_count() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--count")
        .arg("0")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value '0' for '--count <count>': must be a positive integer",
        ));
    assert.failure();
}

#[test]
fn test_invalid_output() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--output")
        .arg("unknown")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value 'unknown' for '--output <name>'",
        ));
    assert.failure();
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
