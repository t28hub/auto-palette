mod adjacency;
mod algorithm;
mod kmeans;
mod segments;
mod snic;

pub(crate) use self::{
    algorithm::{SegmentationAlgorithm, Segmenter},
    segments::{Segment, Segments},
};
