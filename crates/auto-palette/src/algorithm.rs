use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{
    error::Error,
    math::FloatNumber,
    segmentation::{
        DbscanConfig,
        FastDbscanConfig,
        KmeansConfig,
        SegmentationMethod,
        SlicConfig,
        SnicConfig,
    },
};

/// The clustering algorithm to use for color palette extraction.
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use auto_palette::Algorithm;
///
/// let algorithm = Algorithm::from_str("dbscan++").unwrap();
/// assert_eq!(algorithm, Algorithm::DbscanPlusPlus);
/// assert_eq!(algorithm.to_string(), "dbscan++");
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    /// K-means clustering algorithm.
    KMeans,

    /// DBSCAN clustering algorithm.
    #[default]
    Dbscan,

    /// DBSCAN++ clustering algorithm.
    DbscanPlusPlus,

    /// SLIC (Simple Linear Iterative Clustering) algorithm.
    Slic,

    /// SNIC (Simple Non-Iterative Clustering) algorithm.
    Snic,
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kmeans" => Ok(Self::KMeans),
            "dbscan" => Ok(Self::Dbscan),
            "dbscan++" => Ok(Self::DbscanPlusPlus),
            "slic" => Ok(Self::Slic),
            "snic" => Ok(Self::Snic),
            _ => Err(Error::UnknownAlgorithm(s.to_string())),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KMeans => write!(f, "kmeans"),
            Self::Dbscan => write!(f, "dbscan"),
            Self::DbscanPlusPlus => write!(f, "dbscan++"),
            Self::Slic => write!(f, "slic"),
            Self::Snic => write!(f, "snic"),
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
            Algorithm::Dbscan => Self::Dbscan(DbscanConfig::default()),
            Algorithm::DbscanPlusPlus => Self::FastDbscan(FastDbscanConfig::default()),
            Algorithm::Slic => Self::Slic(SlicConfig::default()),
            Algorithm::Snic => Self::Snic(SnicConfig::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::kmeans("kmeans", Algorithm::KMeans)]
    #[case::dbscan("dbscan", Algorithm::Dbscan)]
    #[case::dbscanpp("dbscan++", Algorithm::DbscanPlusPlus)]
    #[case::slic("slic", Algorithm::Slic)]
    #[case::snic("snic", Algorithm::Snic)]
    #[case::kmeans_upper("KMEANS", Algorithm::KMeans)]
    #[case::dbscan_upper("DBSCAN", Algorithm::Dbscan)]
    #[case::dbscanpp_upper("DBSCAN++", Algorithm::DbscanPlusPlus)]
    #[case::slic_upper("SLIC", Algorithm::Slic)]
    #[case::snic_upper("SNIC", Algorithm::Snic)]
    #[case::kmeans_capitalized("Kmeans", Algorithm::KMeans)]
    #[case::dbscan_capitalized("Dbscan", Algorithm::Dbscan)]
    #[case::dbscanpp_capitalized("Dbscan++", Algorithm::DbscanPlusPlus)]
    #[case::slic_capitalized("Slic", Algorithm::Slic)]
    #[case::snic_capitalized("Snic", Algorithm::Snic)]
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
            format!("unsupported algorithm: '{}'", input)
        );
    }

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, "kmeans")]
    #[case::dbscan(Algorithm::Dbscan, "dbscan")]
    #[case::dbscanpp(Algorithm::DbscanPlusPlus, "dbscan++")]
    #[case::slic(Algorithm::Slic, "slic")]
    #[case::snic(Algorithm::Snic, "snic")]
    fn test_fmt(#[case] algorithm: Algorithm, #[case] expected: &str) {
        // Act
        let actual = format!("{}", algorithm);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, SegmentationMethod::Kmeans(KmeansConfig::default()))]
    #[case::dbscan(Algorithm::Dbscan, SegmentationMethod::Dbscan(DbscanConfig::default()))]
    #[case::dbscanpp(
        Algorithm::DbscanPlusPlus,
        SegmentationMethod::FastDbscan(FastDbscanConfig::default())
    )]
    #[case::slic(Algorithm::Slic, SegmentationMethod::Slic(SlicConfig::default()))]
    #[case::snic(Algorithm::Snic, SegmentationMethod::Snic(SnicConfig::default()))]
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
