use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{
    error::Error,
    image::segmentation::{
        DbscanConfig,
        FastDbscanConfig,
        KmeansConfig,
        SegmentationMethod,
        SlicConfig,
        SnicConfig,
    },
    math::FloatNumber,
};

/// The clustering algorithm to use for color palette extraction.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Algorithm {
    /// K-means clustering algorithm.
    KMeans,

    /// DBSCAN clustering algorithm.
    #[default]
    DBSCAN,

    /// DBSCAN++ clustering algorithm.
    DBSCANpp,

    /// SLIC (Simple Linear Iterative Clustering) algorithm.
    SLIC,

    /// SNIC (Simple Non-Iterative Clustering) algorithm.
    SNIC,
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kmeans" => Ok(Self::KMeans),
            "dbscan" => Ok(Self::DBSCAN),
            "dbscan++" => Ok(Self::DBSCANpp),
            "slic" => Ok(Self::SLIC),
            "snic" => Ok(Self::SNIC),
            _ => Err(Error::UnsupportedAlgorithm {
                name: s.to_string(),
            }),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KMeans => write!(f, "kmeans"),
            Self::DBSCAN => write!(f, "dbscan"),
            Self::DBSCANpp => write!(f, "dbscan++"),
            Self::SLIC => write!(f, "slic"),
            Self::SNIC => write!(f, "snic"),
        }
    }
}

impl<T> From<Algorithm> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::KMeans => Self::Kmeans(KmeansConfig::default()),
            Algorithm::DBSCAN => Self::Dbscan(DbscanConfig::default()),
            Algorithm::DBSCANpp => Self::FastDbscan(FastDbscanConfig::default()),
            Algorithm::SLIC => Self::Slic(SlicConfig::default()),
            Algorithm::SNIC => Self::Snic(SnicConfig::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::kmeans("kmeans", Algorithm::KMeans)]
    #[case::dbscan("dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp("dbscan++", Algorithm::DBSCANpp)]
    #[case::slic("slic", Algorithm::SLIC)]
    #[case::snic("snic", Algorithm::SNIC)]
    #[case::kmeans_upper("KMEANS", Algorithm::KMeans)]
    #[case::dbscan_upper("DBSCAN", Algorithm::DBSCAN)]
    #[case::dbscanpp_upper("DBSCAN++", Algorithm::DBSCANpp)]
    #[case::slic_upper("SLIC", Algorithm::SLIC)]
    #[case::snic_upper("SNIC", Algorithm::SNIC)]
    #[case::kmeans_capitalized("Kmeans", Algorithm::KMeans)]
    #[case::dbscan_capitalized("Dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp_capitalized("Dbscan++", Algorithm::DBSCANpp)]
    #[case::slic_capitalized("Slic", Algorithm::SLIC)]
    #[case::snic_capitalized("Snic", Algorithm::SNIC)]
    fn test_from_str(#[case] input: &str, #[case] expected: Algorithm) {
        // Act
        let actual = Algorithm::from_str(input).unwrap();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty("")]
    #[case::invalid("unknown")]
    fn test_from_str_error(#[case] input: &str) {
        // Act
        let actual = Algorithm::from_str(input);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err().to_string(),
            format!("Unsupported algorithm specified: '{}'", input)
        );
    }

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, "kmeans")]
    #[case::dbscan(Algorithm::DBSCAN, "dbscan")]
    #[case::dbscanpp(Algorithm::DBSCANpp, "dbscan++")]
    #[case::slic(Algorithm::SLIC, "slic")]
    #[case::snic(Algorithm::SNIC, "snic")]
    fn test_fmt(#[case] algorithm: Algorithm, #[case] expected: &str) {
        // Act
        let actual = format!("{}", algorithm);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, SegmentationMethod::Kmeans(KmeansConfig::default()))]
    #[case::dbscan(Algorithm::DBSCAN, SegmentationMethod::Dbscan(DbscanConfig::default()))]
    #[case::dbscanpp(
        Algorithm::DBSCANpp,
        SegmentationMethod::FastDbscan(FastDbscanConfig::default())
    )]
    #[case::slic(Algorithm::SLIC, SegmentationMethod::Slic(SlicConfig::default()))]
    #[case::snic(Algorithm::SNIC, SegmentationMethod::Snic(SnicConfig::default()))]
    fn test_into_segmentation_method(
        #[case] algorithm: Algorithm,
        #[case] expected: SegmentationMethod<f64>,
    ) {
        // Act
        let actual: SegmentationMethod<f64> = SegmentationMethod::from(algorithm);

        // Assert
        assert_eq!(actual, expected);
    }
}
