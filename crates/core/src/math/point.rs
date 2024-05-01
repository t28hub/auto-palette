/// Point in an N-dimensional space.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
pub type Point<T, const N: usize> = [T; N];
