/// Point in an N-dimensional space.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
pub type Point<const N: usize> = [f32; N];

/// Point in a 5-dimensional space.
pub type Point5D = Point<5>;
