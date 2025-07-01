use std::{fs, io::Error};

use bytemuck::{Pod, Zeroable};

use crate::{color::GamutKind, FloatNumber};

/// A lookup table that stores the maximum chroma values for each hue in a specific gamut.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChromaLookupTable {
    /// The header containing metadata about the lookup table.
    header: Header,

    /// The body containing the chroma values for each hue.
    body: [f32; 360],
}

impl ChromaLookupTable {
    /// Current version of the LUT format.
    const VERSION: u8 = 1;

    /// The size of the LUT header in bytes.
    const HEADER_SIZE: usize = size_of::<Header>();

    pub fn kind(&self) -> GamutKind {
        GamutKind::try_from(self.header.kind).expect("Invalid gamut kind in LUT")
    }

    pub fn values<T>(&self) -> [T; 360]
    where
        T: FloatNumber,
    {
        self.body.map(|value| T::from_f32(value))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, LutError> {
        if bytes.len() < Self::HEADER_SIZE {
            return Err(LutError::UnexpectedHeaderSize {
                expected: Self::HEADER_SIZE,
                actual: bytes.len(),
            });
        }

        let (header_bytes, body_bytes) = bytes.split_at(Self::HEADER_SIZE);
        let header: &Header = bytemuck::from_bytes(header_bytes);
        if header.version != Self::VERSION {
            return Err(LutError::UnsupportedVersion(header.version));
        }

        let Ok(_kind) = GamutKind::try_from(header.kind) else {
            return Err(LutError::UnsupportedGamutKind(header.kind));
        };

        let size = body_bytes.len();
        if header.size as usize != size {
            return Err(LutError::UnexpectedSize {
                expected: header.size as usize,
                actual: size,
            });
        }

        let checksum = crc32fast::hash(body_bytes);
        if header.checksum != checksum {
            return Err(LutError::ChecksumMismatch {
                expected: header.checksum,
                actual: checksum,
            });
        }

        let body = {
            let mut values = [0.0; 360];
            let bytes: &[f32] = bytemuck::cast_slice(body_bytes);
            values.copy_from_slice(bytes.split_at(360).0);
            values
        };

        Ok(Self {
            header: *header,
            body,
        })
    }

    #[allow(unused)]
    pub fn from_file<P>(path: P) -> Result<Self, LutError>
    where
        P: AsRef<std::path::Path>,
    {
        let bytes = fs::read(path)?;
        Self::from_bytes(&bytes)
    }
}

/// Error type for LUT operations.
#[derive(Debug, thiserror::Error)]
pub enum LutError {
    /// Error when the LUT file cannot be found or read.
    #[error("I/O error: {0}")]
    Io(#[from] Error),

    /// Error when the size of the header is not as expected.
    #[error("Unexpected header size: expected {expected}, got {actual}")]
    UnexpectedHeaderSize { expected: usize, actual: usize },

    /// Error when the version of the LUT file is unsupported.
    #[error("Unsupported LUT version: {0}")]
    UnsupportedVersion(u8),

    /// Error when the kind of gamut is not recognized.
    #[error("Unsupported gamut kind: {0}")]
    UnsupportedGamutKind(u8),

    /// Error when the size of the LUT body does not match the expected size.
    #[error("Unexpected LUT size: expected {expected}, got {actual}")]
    UnexpectedSize { expected: usize, actual: usize },

    /// Error when the checksum of the LUT body does not match the expected checksum.
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: u32, actual: u32 },
}

/// Metadata header for a file in the LUT format.
///
/// # Notes
/// The layout of this struct **must** match the expected binary format of the LUT file.
#[derive(Debug, Clone, Copy, PartialEq, Zeroable)]
#[repr(C)]
struct Header {
    /// The version of the LUT file format.
    version: u8,

    /// The identifier for the gamut type.
    kind: u8,

    /// The size of the LUT body in bytes.
    size: u32,

    /// The checksum hash of the body data.
    checksum: u32,

    /// The reserved bytes for future use or alignment.
    _reserved: [u8; 8],
}

unsafe impl Pod for Header {}
