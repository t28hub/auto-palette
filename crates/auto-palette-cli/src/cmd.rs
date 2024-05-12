use clap::{crate_description, crate_version, Arg, Command};

/// Builds the command line interface.
///
/// # Returns
/// The command line interface.
pub fn build_command() -> Command {
    Command::new("auto-palette")
        .bin_name("auto-palette")
        .about(crate_description!())
        .version(crate_version!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("image")
                .value_name("IMAGE")
                .help("Path to the image file.")
                .long_help("Path to the image file. Supported formats: PNG, JPEG, GIF, BMP, ICO, and TIFF.")
                .required(true)
        )
        .arg(
            Arg::new("algorithm")
                .long("algorithm")
                .short('a')
                .value_name("name")
                .help("Algorithm to use for extracting the palette.")
                .value_parser(["dbscan", "dbscan++", "kmeans"])
                .ignore_case(true)
                .default_value("dbscan++")
        )
        .arg(
            Arg::new("theme")
                .long("theme")
                .short('t')
                .value_name("name")
                .help("Theme to use for extracting the palette.")
                .value_parser(["basic", "vivid", "muted", "light", "dark"])
                .ignore_case(true)
                .default_value("basic")
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('n')
                .value_name("number")
                .help("Number of swatches to extract.")
                .default_value("5")
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command() {
        let command = build_command();
        command.debug_assert();
    }
}
