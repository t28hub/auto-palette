use rustc_hash::FxHashMap;

use crate::next::math::Vector;

/// The largest cube position, small enough that rounding moves a position far less than one cube.
const MAX_POSITION: f64 = 4_294_967_296.0;

/// How much wider than the reach a cube is, well above the rounding of positions up to the largest.
const MARGIN: f64 = 1.0 + 16.0 * MAX_POSITION * f64::EPSILON;

/// Locates the cube of some coordinates.
///
/// # Arguments
/// * `coordinates` - The coordinates to locate.
/// * `size` - The edge length of the cubes, positive.
///
/// # Returns
/// The position of the cube that contains `coordinates`, in edge lengths along each axis.
fn locate_cube<const N: usize>(coordinates: Vector<N>, size: f64) -> [i64; N] {
    coordinates.0.map(|value| (value / size).floor() as i64)
}

/// Indices placed in cubes by their coordinates, for finding near indices in low dimensions.
#[derive(Debug)]
pub(super) struct Grid<const N: usize> {
    /// The edge length of the cubes.
    size: f64,
    /// The indices in each cube.
    cubes: FxHashMap<[i64; N], Vec<usize>>,
}

impl<const N: usize> Grid<N> {
    /// Creates an empty grid that finds the indices within a reach.
    ///
    /// # Arguments
    /// * `reach` - The distance within which `find_near` finds every index, positive.
    /// * `coordinates` - The coordinates to insert or look around, or a set that bounds them.
    ///
    /// # Returns
    /// A grid with no indices, its cubes wide enough that every position stays exact.
    pub(super) fn new(reach: f64, coordinates: impl IntoIterator<Item = Vector<N>>) -> Self {
        let mut extent: f64 = 0.0;
        for vector in coordinates {
            for value in vector.0 {
                extent = extent.max(value.abs());
            }
        }
        let size = reach.max(extent / MAX_POSITION) * MARGIN;
        Self {
            size,
            cubes: FxHashMap::default(),
        }
    }

    /// Inserts an index at some coordinates.
    ///
    /// # Arguments
    /// * `index` - The index to insert.
    /// * `coordinates` - The coordinates of the index.
    pub(super) fn insert(&mut self, index: usize, coordinates: Vector<N>) {
        let cube = locate_cube(coordinates, self.size);
        self.cubes.entry(cube).or_default().push(index);
    }

    /// Removes an index from some coordinates.
    ///
    /// # Arguments
    /// * `index` - The index to remove.
    /// * `coordinates` - The coordinates the index was inserted at.
    pub(super) fn remove(&mut self, index: usize, coordinates: Vector<N>) {
        let cube = locate_cube(coordinates, self.size);
        let indices = self
            .cubes
            .get_mut(&cube)
            .expect("an index is removed from where it was inserted");
        indices.retain(|&other| other != index);
    }

    /// Finds the indices in the cube of some coordinates and the cubes around it.
    ///
    /// # Arguments
    /// * `coordinates` - The coordinates to look around.
    ///
    /// # Returns
    /// The indices, including every one within the reach of `coordinates`.
    pub(super) fn find_near(&self, coordinates: Vector<N>) -> impl Iterator<Item = usize> + '_ {
        // 3^N cubes grow fast, so higher dimensions need a search chosen by N, e.g. brute force.
        let center = locate_cube(coordinates, self.size);
        // Each number from 0 to 3^N - 1 picks an offset of -1, 0, or 1 along each axis.
        (0..3_usize.pow(N as u32))
            .filter_map(move |mut offsets| {
                let mut cube = center;
                for position in &mut cube {
                    *position += (offsets % 3) as i64 - 1;
                    offsets /= 3;
                }
                self.cubes.get(&cube)
            })
            .flatten()
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate_cube_rounds_down() {
        // Act
        let actual = locate_cube(Vector([0.05, -0.05, 0.15]), 0.1);

        // Assert
        assert_eq!(actual, [0, -1, 1]);
    }

    #[test]
    fn test_find_near_finds_indices_in_surrounding_cubes() {
        // Arrange
        let mut grid = Grid::new(0.1, []);
        grid.insert(0, Vector([0.05, 0.05, 0.05]));
        grid.insert(1, Vector([-0.05, 0.15, 0.05]));
        grid.insert(2, Vector([0.25, 0.05, 0.05]));

        // Act
        let mut actual: Vec<usize> = grid.find_near(Vector([0.05, 0.05, 0.05])).collect();

        // Assert
        actual.sort_unstable();
        assert_eq!(actual, [0, 1]);
    }

    #[test]
    fn test_find_near_finds_diagonal_cubes_in_two_dimensions() {
        // Arrange
        let mut grid = Grid::new(0.1, []);
        grid.insert(0, Vector([0.15, 0.15]));
        grid.insert(1, Vector([0.25, 0.05]));

        // Act
        let actual: Vec<usize> = grid.find_near(Vector([0.05, 0.05])).collect();

        // Assert
        assert_eq!(actual, [0]);
    }

    #[test]
    fn test_find_near_finds_indices_far_from_origin() {
        // Arrange
        let far = Vector([2e9, 0.0, 0.0]);
        let mut grid = Grid::new(1e-10, [far]);
        grid.insert(0, far);

        // Act
        let actual: Vec<usize> = grid.find_near(far).collect();

        // Assert
        assert_eq!(actual, [0]);
    }

    #[test]
    fn test_remove_removes_index() {
        // Arrange
        let mut grid = Grid::new(0.1, []);
        grid.insert(0, Vector([0.05, 0.05, 0.05]));
        grid.insert(1, Vector([0.05, 0.05, 0.05]));

        // Act
        grid.remove(0, Vector([0.05, 0.05, 0.05]));

        // Assert
        let actual: Vec<usize> = grid.find_near(Vector([0.05, 0.05, 0.05])).collect();
        assert_eq!(actual, [1]);
    }
}
