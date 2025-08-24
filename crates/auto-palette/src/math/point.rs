/// Point in an N-dimensional space.
///
/// This type alias represents a point as a fixed-size array of floating-point values.
///
/// # Type Parameters
/// * `T` - The floating point type (typically `f32` or `f64`).
/// * `N` - The number of dimensions.
pub type Point<T, const N: usize> = [T; N];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        // 2D point
        let point_2d: Point<f32, 2> = [1.0, 2.0];
        assert_eq!(point_2d[0], 1.0);
        assert_eq!(point_2d[1], 2.0);

        // 3D point
        let point_3d: Point<f64, 3> = [1.0, 2.0, 3.0];
        assert_eq!(point_3d[0], 1.0);
        assert_eq!(point_3d[1], 2.0);
        assert_eq!(point_3d[2], 3.0);

        // 5D point
        let point_5d: Point<f32, 5> = [10.0, 20.0, 30.0, 100.0, 200.0];
        assert_eq!(point_5d.len(), 5);
    }

    #[test]
    fn test_point_modification() {
        // Arrange
        let mut point: Point<f32, 3> = [0.0; 3];

        // Act
        point[0] = 1.0;
        point[1] = 2.0;
        point[2] = 3.0;

        // Assert
        assert_eq!(point, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_point_iteration() {
        // Arrange
        let point: Point<f32, 3> = [1.0, 2.0, 3.0];

        // Act & Assert
        let sum: f32 = point.iter().sum();
        assert_eq!(sum, 6.0);

        let squared: Vec<f32> = point.iter().map(|&x| x * x).collect();
        assert_eq!(squared, vec![1.0, 4.0, 9.0]);
    }
}
