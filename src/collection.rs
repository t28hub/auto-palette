use crate::color_trait::Color;
use crate::delta_e::DeltaE::CIE2000;
use crate::math::graph::edge::Edge;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::{Float, Number};
use crate::Swatch;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

/// Struct representing a collection of swatches.
///
/// # Type Parameters
/// * `C` - The type of color.
#[derive(Debug)]
pub struct Collection<F: Float + Default, C: Color> {
    root: Item<F, C>,
    swatches: Vec<Swatch<C>>,
}

impl<F, C> Collection<F, C>
where
    F: Float + Default,
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
        let n_swatches = swatches.len();
        let mut items = HashMap::<usize, Item<F, C>>::new();
        let mut heap = BinaryHeap::new();
        swatches.iter().enumerate().for_each(|(i, swatch_i)| {
            items.insert(i, Item::new(swatch_i.clone()));

            for (j, swatch_j) in swatches.iter().enumerate().take(i) {
                let distance = distance(swatch_i, &swatches[j]);
                heap.push(Reverse(WeightedEdge::new(i, j, distance)));
            }
        });

        let mut next_label = n_swatches;
        while items.len() > 1 {
            let Some(Reverse(edge)) = heap.pop() else {
                break;
            };

            let Some(item1) = items.get(&edge.u()) else {
                continue;
            };
            let Some(item2) = items.get(&edge.v()) else {
                continue;
            };

            let merged_swatch = {
                let swatch1 = item1.swatch();
                let swatch2 = item2.swatch();
                let population1 = F::from_usize(swatch1.population());
                let population2 = F::from_usize(swatch2.population());
                let fraction = population1 / (population1 + population2);
                Swatch::new(
                    swatch1.color().mix(swatch2.color(), fraction),
                    swatch1.position(),
                    swatch1.population() + swatch2.population(),
                )
            };

            let merged_item = Item {
                left: Some(Box::new(item1.clone())),
                right: Some(Box::new(item2.clone())),
                swatch: merged_swatch,
                distance: edge.weight(),
            };

            items.remove(&edge.u());
            items.remove(&edge.v());

            items.iter().for_each(|(label, item)| {
                let distance = distance(merged_item.swatch(), item.swatch());
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            items.insert(next_label, merged_item);
            next_label += 1;
        }
        Self {
            swatches,
            root: items.values().take(1).next().unwrap().clone(),
        }
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
        let mut heap = BinaryHeap::new();
        heap.push(&self.root);
        while heap.len() < n {
            let Some(item) = heap.pop() else {
                break;
            };

            if let Some(left) = &item.left {
                heap.push(left);
            }
            if let Some(right) = &item.right {
                heap.push(right);
            }
        }

        let mut swatches: Vec<_> = heap.iter().map(|item| item.swatch.clone()).collect();
        swatches.sort_unstable_by_key(|swatch| Reverse(swatch.population()));
        swatches
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Item<F: Float + Default, C: Color> {
    left: Option<Box<Item<F, C>>>,
    right: Option<Box<Item<F, C>>>,
    swatch: Swatch<C>,
    distance: F,
}

impl<F, C> Item<F, C>
where
    F: Float + Default,
    C: Color + PartialEq,
{
    #[must_use]
    fn new(swatch: Swatch<C>) -> Self {
        Self {
            left: None,
            right: None,
            swatch,
            distance: F::zero(),
        }
    }

    #[must_use]
    fn swatch(&self) -> &Swatch<C> {
        &self.swatch
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

#[inline]
#[must_use]
fn distance<C: Color>(swatch1: &Swatch<C>, swatch2: &Swatch<C>) -> C::F {
    let color1 = swatch1.color();
    let color2 = swatch2.color();
    let delta_e = color1.delta_e(color2, CIE2000);
    let weight = swatch1.population() + swatch2.population();
    delta_e * C::F::from_usize(weight)
}
