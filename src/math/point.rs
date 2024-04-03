/// Point represents a point in N-dimensional space.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
pub type Point<const N: usize> = [f32; N];

/// Point3D represents a point in 3-dimensional space.
pub type Point3D = Point<3>;
