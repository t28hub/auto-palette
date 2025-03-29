use std::{borrow::Cow, io::Cursor};

use arboard::Clipboard;
use assert_cmd::Command;
use image::io::Reader as ImageReader;
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
#[ignore]
// ignored by default since it interacts with the system clipboard
fn test_using_clipboard_as_input() {
    let mut clipboard = Clipboard::new().expect("should've set up access to system clipboard");
    let bytes = std::fs::read("../../gfx/olympic_logo.png").expect("should've read file contents");

    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .expect("should've guessed image format");
    let image = reader.decode().expect("should've decoded image");
    let image_data = arboard::ImageData {
        width: image.width() as usize,
        height: image.height() as usize,
        bytes: Cow::from(image.as_bytes()),
    };
    clipboard
        .set_image(image_data)
        .expect("should've placed image to system clipboard");

    let assert = auto_palette()
        .arg("--clipboard")
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
fn test_missing_input() {
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
            "failed to open the image file \"invalid.png\"",
        ));
    assert.failure();
}

#[test]
fn test_multiple_inputs() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--clipboard")
        .assert()
        .stderr(predicate::str::contains(
            "only one input source can be provided",
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
