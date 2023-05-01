use crate::color_trait::Color;
use crate::math::graph::edge::Edge;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::Float;
use crate::Swatch;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Struct representing a collection of swatches.
///
/// # Type Parameters
/// * `C` - The type of color.
#[derive(Debug)]
pub struct Collection<C: Color> {
    swatches: Vec<Swatch<C>>,
}

impl<F, C> Collection<C>
where
    F: Float,
    C: Color<F = F>,
{
    /// Creates a new `Collection` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches to use for this collection.
    ///
    /// # Returns
    /// A new `Collection` instance.
    #[must_use]
    pub fn new(swatches: Vec<Swatch<C>>) -> Self {
        Self { swatches }
    }

    /// Return the number of swatches in this collection.
    ///
    /// # Returns
    /// The number of swatches in this collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.swatches.len()
    }

    /// Return whether this collection is empty.
    ///
    /// # Returns
    /// Whether this collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swatches.is_empty()
    }

    /// Return the swatches of this collection.
    ///
    /// # Returns
    /// The swatches of this collection.
    #[must_use]
    pub fn swatches(&self) -> &[Swatch<C>] {
        &self.swatches
    }

    /// Finds the n best swatches in this collection.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    ///
    /// # Returns
    /// The n best swatches in this collection.
    #[must_use]
    pub fn find_best_swatches(&self, n: usize) -> Vec<Swatch<C>> {
        let mut candidates = HashMap::<usize, Swatch<C>>::new();
        let mut heap = BinaryHeap::new();
        self.swatches.iter().enumerate().for_each(|(i, swatch_i)| {
            candidates.insert(i, swatch_i.clone());

            for (j, swatch_j) in self.swatches.iter().enumerate().take(i) {
                let distance = swatch_i.distance(swatch_j);
                heap.push(Reverse(WeightedEdge::new(i, j, distance)));
            }
        });

        let mut next_label = self.swatches.len();
        while candidates.len() > n {
            let Some(Reverse(edge)) = heap.pop() else {
                break;
            };

            let Some(swatch1) = candidates.get(&edge.u()) else {
                continue;
            };
            let Some(swatch2) = candidates.get(&edge.v()) else {
                continue;
            };

            let new_swatch = swatch1.combine(swatch2);
            candidates.iter().for_each(|(label, swatch)| {
                if label == &edge.u() || label == &edge.v() {
                    return;
                }

                let distance1: F = swatch.distance(swatch1);
                let distance2: F = swatch.distance(swatch2);
                let distance: F = distance1.max(distance2);
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            candidates.remove(&edge.u());
            candidates.remove(&edge.v());
            candidates.insert(next_label, new_swatch);
            next_label += 1;
        }
        candidates.into_values().collect()
    }
}
