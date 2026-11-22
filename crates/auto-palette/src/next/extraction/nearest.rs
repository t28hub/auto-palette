/// Returns the source index nearest to the center of a scaled index.
///
/// # Arguments
/// * `index` - The scaled index.
/// * `target` - The number of scaled pixels along one side.
/// * `source` - The number of source pixels along the same side.
///
/// # Returns
/// The source index whose pixel contains the center of the scaled pixel.
pub(super) fn nearest(index: u32, target: u32, source: u32) -> u32 {
    ((2 * u64::from(index) + 1) * u64::from(source) / (2 * u64::from(target))) as u32
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::three_into_one(0, 1, 3, 1)]
    #[case::four_into_two_first(0, 2, 4, 1)]
    #[case::four_into_two_second(1, 2, 4, 3)]
    fn test_nearest(
        #[case] index: u32,
        #[case] target: u32,
        #[case] source: u32,
        #[case] expected: u32,
    ) {
        // Act
        let actual = nearest(index, target, source);

        // Assert
        assert_eq!(actual, expected);
    }
}
