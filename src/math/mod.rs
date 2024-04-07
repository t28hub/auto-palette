pub mod clustering;
mod metrics;
mod neighbors;
mod number;
mod point;

pub use metrics::DistanceMetric;
pub use number::Normalizable;
pub use point::{Point, Point5D};
