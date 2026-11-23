use rustc_hash::FxHashMap;

use crate::next::math::Vector;

/// The position of a cube, in edge lengths along each axis.
type Cube = [i64; 3];

/// Locates the cube of some coordinates.
///
/// # Arguments
/// * `coordinates` - The coordinates to locate.
/// * `size` - The edge length of the cubes, positive and finite.
///
/// # Returns
/// The cube that contains `coordinates`.
fn locate_cube(coordinates: Vector<3>, size: f64) -> Cube {
    coordinates.0.map(|value| (value / size).floor() as i64)
}

/// Indices placed in cubes by their coordinates, for finding the indices near some coordinates.
#[derive(Debug)]
pub(super) struct Grid {
    /// The edge length of the cubes.
    size: f64,
    /// The indices in each cube.
    cubes: FxHashMap<Cube, Vec<usize>>,
}

impl Grid {
    /// Creates an empty grid.
    ///
    /// # Arguments
    /// * `size` - The edge length of the cubes, positive and finite.
    ///
    /// # Returns
    /// A grid with no indices.
    pub(super) fn new(size: f64) -> Self {
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
    pub(super) fn insert(&mut self, index: usize, coordinates: Vector<3>) {
        let cube = locate_cube(coordinates, self.size);
        self.cubes.entry(cube).or_default().push(index);
    }

    /// Removes an index from some coordinates.
    ///
    /// # Arguments
    /// * `index` - The index to remove.
    /// * `coordinates` - The coordinates the index was inserted at.
    pub(super) fn remove(&mut self, index: usize, coordinates: Vector<3>) {
        let cube = locate_cube(coordinates, self.size);
        let indices = self
            .cubes
            .get_mut(&cube)
            .expect("an index is removed from where it was inserted");
        indices.retain(|&other| other != index);
    }

    /// Finds the indices in the cube of some coordinates and the 26 cubes around it.
    ///
    /// # Arguments
    /// * `coordinates` - The coordinates to look around.
    ///
    /// # Returns
    /// The indices, including every one closer than the edge length to `coordinates`.
    pub(super) fn find_near(&self, coordinates: Vector<3>) -> impl Iterator<Item = usize> + '_ {
        const OFFSETS: [i64; 3] = [-1, 0, 1];
        let [x, y, z] = locate_cube(coordinates, self.size);
        OFFSETS
            .into_iter()
            .flat_map(|dx| OFFSETS.into_iter().map(move |dy| (dx, dy)))
            .flat_map(|(dx, dy)| OFFSETS.into_iter().map(move |dz| (dx, dy, dz)))
            .filter_map(move |(dx, dy, dz)| {
                let cube = [x.checked_add(dx)?, y.checked_add(dy)?, z.checked_add(dz)?];
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
        let mut grid = Grid::new(0.1);
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
    fn test_remove_removes_index() {
        // Arrange
        let mut grid = Grid::new(0.1);
        grid.insert(0, Vector([0.05, 0.05, 0.05]));
        grid.insert(1, Vector([0.05, 0.05, 0.05]));

        // Act
        grid.remove(0, Vector([0.05, 0.05, 0.05]));

        // Assert
        let actual: Vec<usize> = grid.find_near(Vector([0.05, 0.05, 0.05])).collect();
        assert_eq!(actual, [1]);
    }
}
