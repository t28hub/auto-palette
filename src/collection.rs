use crate::color_trait::Color;
use crate::math::graph::edge::Edge;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::Float;
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

    #[must_use]
    pub fn take(&self, n: usize) -> Vec<Swatch<C>> {
        let mut items = HashMap::<usize, Item<F, C>>::new();
        let mut heap = BinaryHeap::new();
        self.swatches.iter().enumerate().for_each(|(i, swatch_i)| {
            items.insert(i, Item::from(swatch_i.clone()));

            for (j, swatch_j) in self.swatches.iter().enumerate().take(i) {
                let distance = swatch_i.distance(swatch_j);
                heap.push(Reverse(WeightedEdge::new(i, j, distance)));
            }
        });

        let mut next_label = self.swatches.len();
        while items.len() > n {
            let Some(Reverse(edge)) = heap.pop() else {
                break;
            };

            let Some(swatch1) = items.get(&edge.u()).map(|item| item.swatch()) else {
                continue;
            };
            let Some(swatch2) = items.get(&edge.v()).map(|item| item.swatch()) else {
                continue;
            };

            let new_swatch = swatch1.combine(swatch2);
            let merged_item = Item::new(new_swatch, edge.u(), edge.v(), edge.weight());
            items.iter().for_each(|(label, item)| {
                if label == &edge.u() || label == &edge.v() {
                    return;
                }

                let distance1: F = item.swatch().distance(swatch1);
                let distance2: F = item.swatch().distance(swatch2);
                let distance: F = distance1.max(distance2);
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            items.remove(&edge.u());
            items.remove(&edge.v());
            items.insert(next_label, merged_item);
            next_label += 1;
        }

        let mut swatches: Vec<_> = items
            .values()
            .map(|item| item.swatch())
            .cloned()
            .collect();
        swatches.sort_unstable_by_key(|swatch| Reverse(swatch.population()));
        swatches
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Item<F: Float, C: Color> {
    swatch: Swatch<C>,
    left: Option<usize>,
    right: Option<usize>,
    distance: F,
}

impl<F, C> Item<F, C>
where
    F: Float,
    C: Color + PartialEq,
{
    #[must_use]
    fn new(swatch: Swatch<C>, left: usize, right: usize, distance: F) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            swatch,
            distance,
        }
    }

    #[must_use]
    fn swatch(&self) -> &Swatch<C> {
        &self.swatch
    }
}

impl<F, C> From<Swatch<C>> for Item<F, C>
where
    F: Float,
    C: Color,
{
    #[must_use]
    fn from(swatch: Swatch<C>) -> Self {
        Self {
            swatch,
            left: None,
            right: None,
            distance: F::zero(),
        }
    }
}

impl<F, C> Eq for Item<F, C>
where
    F: Float + Default,
    C: Color,
{
}

impl<F, C> PartialOrd<Self> for Item<F, C>
where
    F: Float + Default,
    C: Color,
{
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<F, C> Ord for Item<F, C>
where
    F: Float + Default,
    C: Color,
{
    #[inline]
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
