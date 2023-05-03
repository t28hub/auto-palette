use crate::color_trait::Color;
use crate::lab::Lab;
use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::gmeans::algorithm::Gmeans;
use crate::math::distance::Distance;
use crate::math::graph::edge::Edge;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::{Float, Normalize, Number};
use crate::math::point::Point3;
use crate::white_point::D65;
use crate::Swatch;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

/// Struct representing a collection of swatches.
///
/// # Type Parameters
/// * `C` - The type of color.
#[derive(Debug)]
pub struct Collection<C: Color> {
    swatches: Vec<Swatch<C>>,
}

impl<C> Collection<C>
where
    C: Color,
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

    #[must_use]
    pub fn find(&self, n: usize) -> Vec<Swatch<C>> {
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

            let new_swatch = {
                let population1: C::F = C::F::from_usize(swatch1.population());
                let population2: C::F = C::F::from_usize(swatch2.population());
                let fraction = population2 / (population1 + population2);

                let color = swatch1.color().mix(swatch2.color(), fraction);
                let position = if swatch1.population() > swatch2.population() {
                    swatch1.position()
                } else {
                    swatch2.position()
                };
                let population = swatch1.population() + swatch2.population();
                Swatch::new(color, position, population)
            };

            candidates.iter().for_each(|(label, swatch)| {
                if label == &edge.u() || label == &edge.v() {
                    return;
                }

                // Use UPGMA to calculate the distance between the new swatch and the other swatches.
                let distance1 = swatch1.distance(swatch);
                let distance2 = swatch2.distance(swatch);
                let population1: C::F = C::F::from_usize(swatch1.population());
                let population2: C::F = C::F::from_usize(swatch2.population());
                let distance = (distance1 * population1 + distance2 * population2)
                    / (population1 + population2);
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            candidates.remove(&edge.u());
            candidates.remove(&edge.v());
            candidates.insert(next_label, new_swatch);
            next_label += 1;
        }
        candidates.into_values().collect()
    }

    /// Finds the n best swatches with the given score function.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    /// * `score_fn` - The score function to use.
    ///
    /// # Returns
    /// The found n best swatches.
    #[must_use]
    pub fn find_with_score<SF>(&self, n: usize, score_fn: SF) -> Vec<Swatch<C>>
    where
        SF: Fn(&Swatch<C>) -> C::F,
    {
        let points: Vec<_> = self
            .swatches
            .iter()
            .map(|swatch| {
                let color = swatch.color().to_lab();
                let l = color
                    .l
                    .normalize(Lab::<C::F, D65>::min_l(), Lab::<C::F, D65>::max_l());
                let a = color
                    .a
                    .normalize(Lab::<C::F, D65>::min_a(), Lab::<C::F, D65>::max_a());
                let b = color
                    .b
                    .normalize(Lab::<C::F, D65>::min_b(), Lab::<C::F, D65>::max_b());
                Point3(l, a, b)
            })
            .collect();
        let gmeans = Gmeans::new(64, 10, 2, C::F::from_f64(1e-4), Distance::SquaredEuclidean);
        let model = gmeans.train(&points);

        let clusters = model.clusters();
        let mut swatches: Vec<_> = clusters
            .iter()
            .filter_map(|cluster| {
                let membership = cluster.membership();
                let first_swatch = membership.first().map(|i| &self.swatches[*i]);
                let mut best_swatch = if let Some(swatch) = first_swatch {
                    swatch.clone()
                } else {
                    return None;
                };

                membership.iter().skip(1).for_each(|i| {
                    let swatch = &self.swatches[*i];
                    if score_fn(swatch) < score_fn(&best_swatch) {
                        return;
                    }

                    best_swatch = {
                        let color = swatch.color().clone();
                        let position = swatch.position();
                        let population = swatch.population() + best_swatch.population();
                        Swatch::new(color, position, population)
                    };
                });
                Some(best_swatch)
            })
            .collect();

        swatches.sort_unstable_by(|swatch1, swatch2| {
            let weight1 = score_fn(swatch1);
            let weight2 = score_fn(swatch2);
            weight1
                .partial_cmp(&weight2)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        swatches.iter().take(n).cloned().collect()
    }
}
