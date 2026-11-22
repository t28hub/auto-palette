use super::segments::SegmentId;
use crate::next::image::Size;

/// The segments that touch, in compressed sparse rows.
#[derive(Debug)]
pub(crate) struct Adjacency {
    /// Where the neighbors of each segment start in `neighbors`, with the end appended.
    offsets: Vec<u32>,
    /// The neighbors of every segment, sorted by number within each segment.
    neighbors: Vec<Neighbor>,
}

/// A segment next to another, and how long the two touch.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the segment statistics, which are not implemented yet"
    )
)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Neighbor {
    /// The neighboring segment.
    pub(crate) id: SegmentId,
    /// The number of pixel edges the two share.
    pub(crate) length: u32,
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "called by the segment statistics, which are not implemented yet"
    )
)]
impl Adjacency {
    /// Finds the segments that share an edge between two pixels.
    ///
    /// # Arguments
    /// * `size` - The size of the image.
    /// * `labels` - The segment of each pixel, row by row, or `None` for a pixel in no segment.
    /// * `count` - The number of segments.
    ///
    /// # Returns
    /// The neighbors of each of the `count` segments.
    ///
    /// # Panics
    /// Panics if `labels` and the pixels differ in number, or a label is not below `count`.
    pub(crate) fn of_labels(size: Size, labels: &[Option<SegmentId>], count: usize) -> Self {
        assert_eq!(
            labels.len(),
            size.pixel_count(),
            "the number of labels does not match the number of pixels"
        );
        assert!(
            labels.iter().flatten().all(|id| id.index() < count),
            "a label is not below the number of segments"
        );
        let edges = find_shared_edges(size, labels);
        let borders = count_shared_edges(edges);

        let mut offsets = vec![0; count + 1];
        let mut neighbors = Vec::with_capacity(borders.len());
        for border in borders {
            offsets[border.segment.index() + 1] += 1;
            neighbors.push(border.neighbor);
        }
        for index in 1..offsets.len() {
            offsets[index] += offsets[index - 1];
        }
        Self { offsets, neighbors }
    }

    /// Returns the segments that touch a segment.
    ///
    /// # Arguments
    /// * `id` - The segment, below the number of segments.
    ///
    /// # Returns
    /// The neighbors of the segment, sorted by number.
    ///
    /// # Panics
    /// Panics if `id` is not below the number of segments.
    pub(crate) fn neighbors(&self, id: SegmentId) -> impl Iterator<Item = Neighbor> + '_ {
        let start = self.offsets[id.index()] as usize;
        let end = self.offsets[id.index() + 1] as usize;
        self.neighbors[start..end].iter().copied()
    }
}

/// The pixel edges that a segment shares with one neighbor.
#[derive(Debug)]
struct Border {
    /// The segment on this side.
    segment: SegmentId,
    /// The segment on the other side, with the length of the border.
    neighbor: Neighbor,
}

/// Finds the pixel edges that two segments share.
///
/// # Arguments
/// * `size` - The size of the image.
/// * `labels` - The segment of each pixel, row by row, or `None` for a pixel in no segment.
///
/// # Returns
/// The two segments of each shared edge, once in each order.
fn find_shared_edges(size: Size, labels: &[Option<SegmentId>]) -> Vec<(SegmentId, SegmentId)> {
    let (width, height) = (size.width as usize, size.height as usize);
    let mut edges = Vec::new();
    for (index, label) in labels.iter().enumerate() {
        let Some(id) = *label else {
            continue;
        };
        let (column, row) = (index % width, index / width);
        // Only right and below, so that each edge is found once, from its left or top pixel.
        let right = (column + 1 < width).then(|| index + 1);
        let below = (row + 1 < height).then(|| index + width);
        for neighbor_index in [right, below].into_iter().flatten() {
            let Some(other) = labels[neighbor_index] else {
                continue;
            };
            if other == id {
                continue;
            }
            edges.push((id, other));
            edges.push((other, id));
        }
    }
    edges
}

/// Counts the pixel edges that each segment shares with each neighbor.
///
/// # Arguments
/// * `edges` - The two segments of each shared edge, in any order.
///
/// # Returns
/// The border between each segment and each neighbor, sorted by segment and then neighbor.
fn count_shared_edges(mut edges: Vec<(SegmentId, SegmentId)>) -> Vec<Border> {
    edges.sort_unstable();
    edges
        .chunk_by(|a, b| a == b)
        .map(|run| {
            // Every edge in a run is the same pair.
            let (segment, neighbor) = run[0];
            Border {
                segment,
                neighbor: Neighbor {
                    id: neighbor,
                    length: run.len() as u32,
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Finds the touching segments in an image of `W` columns with `count` segments.
    fn adjacency<const W: usize>(rows: &[[Option<u32>; W]], count: usize) -> Adjacency {
        let labels: Vec<Option<SegmentId>> = rows
            .iter()
            .flatten()
            .map(|label| label.map(SegmentId::new))
            .collect();
        Adjacency::of_labels(Size::new(W as u32, rows.len() as u32), &labels, count)
    }

    /// Returns the neighbors of segment `id` as numbers and lengths.
    fn neighbors(adjacency: &Adjacency, id: u32) -> Vec<(usize, u32)> {
        adjacency
            .neighbors(SegmentId::new(id))
            .map(|neighbor| (neighbor.id.index(), neighbor.length))
            .collect()
    }

    #[test]
    fn test_of_labels_finds_horizontal_neighbors() {
        // Act
        let actual = adjacency(&[[Some(0), Some(1)]], 2);

        // Assert
        assert_eq!(neighbors(&actual, 0), [(1, 1)]);
        assert_eq!(neighbors(&actual, 1), [(0, 1)]);
    }

    #[test]
    fn test_of_labels_finds_vertical_neighbors() {
        // Act
        let actual = adjacency(&[[Some(0)], [Some(1)]], 2);

        // Assert
        assert_eq!(neighbors(&actual, 0), [(1, 1)]);
        assert_eq!(neighbors(&actual, 1), [(0, 1)]);
    }

    #[test]
    fn test_of_labels_counts_shared_edges() {
        // Act: segments 0 and 1 share two edges, one in each row.
        let actual = adjacency(&[[Some(0), Some(1)], [Some(0), Some(1)]], 2);

        // Assert
        assert_eq!(neighbors(&actual, 0), [(1, 2)]);
    }

    #[test]
    fn test_of_labels_ignores_diagonal_pixels() {
        // Act: segments 1 and 2 meet only at a corner.
        let actual = adjacency(&[[Some(0), Some(1)], [Some(2), Some(0)]], 3);

        // Assert
        assert_eq!(neighbors(&actual, 1), [(0, 2)]);
        assert_eq!(neighbors(&actual, 2), [(0, 2)]);
    }

    #[test]
    fn test_of_labels_skips_unlabeled_pixels() {
        // Act
        let actual = adjacency(&[[Some(0), None, Some(1)]], 2);

        // Assert
        assert!(neighbors(&actual, 0).is_empty());
        assert!(neighbors(&actual, 1).is_empty());
    }

    #[test]
    #[should_panic(expected = "the number of labels does not match the number of pixels")]
    fn test_of_labels_rejects_wrong_label_count() {
        // Act
        Adjacency::of_labels(Size::new(2, 1), &[Some(SegmentId::new(0))], 1);
    }

    #[test]
    #[should_panic(expected = "a label is not below the number of segments")]
    fn test_of_labels_rejects_label_beyond_count() {
        // Act
        adjacency(&[[Some(0), Some(1)]], 1);
    }

    /// Creates the edge between segments `a` and `b`.
    fn edge(a: u32, b: u32) -> (SegmentId, SegmentId) {
        (SegmentId::new(a), SegmentId::new(b))
    }

    #[test]
    fn test_count_shared_edges_counts_each_pair() {
        // Act
        let actual = count_shared_edges(vec![edge(0, 1), edge(0, 1), edge(0, 2)]);

        // Assert
        let lengths: Vec<u32> = actual.iter().map(|border| border.neighbor.length).collect();
        assert_eq!(lengths, [2, 1]);
    }

    #[test]
    fn test_count_shared_edges_sorts_by_segment_and_neighbor() {
        // Act
        let actual = count_shared_edges(vec![edge(1, 0), edge(0, 2), edge(0, 1)]);

        // Assert
        let sides: Vec<(usize, usize)> = actual
            .iter()
            .map(|border| (border.segment.index(), border.neighbor.id.index()))
            .collect();
        assert_eq!(sides, [(0, 1), (0, 2), (1, 0)]);
    }

    #[test]
    fn test_neighbors_sorted_by_number() {
        // Arrange
        let adjacency = adjacency(&[[Some(2), Some(0), Some(1)]], 3);

        // Act
        let actual = neighbors(&adjacency, 0);

        // Assert
        assert_eq!(actual, [(1, 1), (2, 1)]);
    }

    #[test]
    fn test_neighbors_of_segment_without_pixels() {
        // Arrange
        let adjacency = adjacency(&[[Some(0), Some(2)]], 3);

        // Act
        let actual = neighbors(&adjacency, 1);

        // Assert
        assert!(actual.is_empty());
    }
}
