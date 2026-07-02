use std::{borrow::Cow, io::Cursor, str::FromStr};

use arboard::Clipboard;
use assert_cmd::Command;
use auto_palette::color::Color;
use image::ImageReader;
use predicates::prelude::*;
use rstest::rstest;

/// Returns the auto-palette command.
///
/// # Returns
/// The auto-palette command.
#[must_use]
fn auto_palette() -> Command {
    Command::cargo_bin("auto-palette-cli").unwrap()
}

/// Extracts all `#RRGGBB` hex color codes from the CLI output.
fn extract_hex_colors(output: &str) -> Vec<Color<f64>> {
    let bytes = output.as_bytes();
    let mut colors = Vec::new();
    for (index, _) in output.match_indices('#') {
        let Some(candidate) = bytes.get(index..index + 7) else {
            continue;
        };
        if candidate[1..].iter().all(u8::is_ascii_hexdigit) {
            if let Ok(color) = Color::from_str(std::str::from_utf8(candidate).unwrap()) {
                colors.push(color);
            }
        }
    }
    colors
}

/// Asserts that every expected hex color has a perceptually close match
/// (delta-E based) among the colors printed by the CLI. Unlike exact string
/// matching, this tolerates small shifts in the extracted palette.
fn assert_output_contains_colors(output: &str, expected: &[&str], tolerance: f64) {
    let actual_colors = extract_hex_colors(output);
    for hex in expected {
        let expected_color = Color::<f64>::from_str(hex).expect("Invalid hex color format");
        let matched = actual_colors
            .iter()
            .any(|actual| actual.delta_e(&expected_color) < tolerance);
        assert!(
            matched,
            "expected a color close to {hex} (tolerance {tolerance}) in output:\n{output}"
        );
    }
}

#[test]
fn test_cli() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--algorithm")
        .arg("dbscan")
        .arg("--count")
        .arg("6")
        .arg("--output-format")
        .arg("table")
        .arg("--no-resize")
        .assert()
        .stdout(predicate::str::contains("Extracted 6 swatch(es) in"))
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    assert_output_contains_colors(
        &stdout,
        &[
            "#FFFFFF", "#0081C8", "#EE334E", "#000000", "#00A651", "#FCB131",
        ],
        10.0,
    );
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
        .stdout(predicate::str::contains("Extracted 6 swatch(es) in"))
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    assert_output_contains_colors(
        &stdout,
        &["#FFFFFF", "#0081C8", "#FCB131", "#EE334E", "#000000"],
        10.0,
    );
}

#[test]
fn test_missing_input() {
    let assert = auto_palette().assert().stderr(predicate::str::contains(
        "🎨 CLI tool to extract a prominent color palette from an image.",
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

#[rstest]
#[case::dbscan("dbscan")]
#[case::dbscanpp("dbscan++")]
#[case::kmeans("kmeans")]
#[case::slic("slic")]
#[case::snic("snic")]
fn test_algorithm(#[case] algorithm: &str) {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--algorithm")
        .arg(algorithm)
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
            "invalid value 'unknown' for '--algorithm <ALGORITHM>'",
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
            "invalid value 'unknown' for '--theme <THEME>'",
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
            "invalid value '0' for '--count <N>': must be a positive integer",
        ));
    assert.failure();
}

#[test]
fn test_color_space() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--color-space")
        .arg("rgb")
        .assert();
    assert.success();
}

#[test]
fn test_invalid_color_space() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--color-space")
        .arg("unknown")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value 'unknown' for '--color-space <SPACE>'",
        ));
    assert.failure();
}

#[test]
fn test_invalid_output_format() {
    let assert = auto_palette()
        .arg("../../gfx/olympic_logo.png")
        .arg("--output-format")
        .arg("unknown")
        .assert()
        .stderr(predicate::str::contains(
            "invalid value 'unknown' for '--output-format <FORMAT>'",
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
