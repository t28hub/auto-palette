/// SplitMix64, a small pseudorandom number generator.
///
/// From <https://prng.di.unimi.it/splitmix64.c>.
pub(super) struct SplitMix64 {
    /// The state, which advances by a fixed step for each number.
    state: u64,
}

impl SplitMix64 {
    /// Creates a generator from a seed.
    ///
    /// # Arguments
    /// * `seed` - The starting state.
    ///
    /// # Returns
    /// A generator that generates the same sequence for the same seed.
    pub(super) const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generates the next 64-bit number.
    ///
    /// # Returns
    /// The next 64-bit number.
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Generates the next number from 0.0 up to, but not including, 1.0.
    ///
    /// # Returns
    /// The next number, from the top 53 bits of [`Self::next_u64`].
    pub(super) fn next_f64(&mut self) -> f64 {
        let top_53_bits = self.next_u64() >> 11;
        let scale = (1u64 << 53) as f64;
        top_53_bits as f64 / scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_same_seed() {
        // Act
        let first = SplitMix64::new(1).next_u64();
        let second = SplitMix64::new(1).next_u64();

        // Assert
        assert_eq!(first, second);
    }

    #[test]
    fn test_new_different_seeds() {
        // Act
        let first = SplitMix64::new(1).next_u64();
        let second = SplitMix64::new(2).next_u64();

        // Assert
        assert_ne!(first, second);
    }

    #[test]
    fn test_next_u64() {
        // Arrange
        let mut random = SplitMix64::new(0);

        // Act
        let actual = random.next_u64();

        // Assert: the first output for seed 0 in the reference implementation.
        assert_eq!(actual, 0xe220_a839_7b1d_cdaf);
    }

    #[test]
    fn test_next_f64() {
        // Arrange
        let mut random = SplitMix64::new(0);

        // Act
        let actual: Vec<f64> = (0..1000).map(|_| random.next_f64()).collect();

        // Assert
        assert!(actual.iter().all(|value| (0.0..1.0).contains(value)));
    }
}
