#![allow(dead_code)]

use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use anyhow::{ensure, Error};

const BIT_FIVE: u8 = 32;

/// 4-byte chunk type code for PNG files
/// See spec for details: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChunkType {
    name: [u8; 4],
}

impl ChunkType {
    /// The bytes contained in the struct
    pub fn bytes(&self) -> [u8; 4] {
        self.name
    }

    /// Checks to see if chunk type code is valid
    pub fn is_valid(&self) -> bool {
        // We could also check for ASCII alphabetic here
        // but since that's done on construction there is no need
        self.is_reserved_bit_valid()
    }

    /// Checks first byte to see if chunk type is critical
    pub fn is_critical(&self) -> bool {
        self.name[0] & BIT_FIVE != BIT_FIVE
    }

    /// Checks second byte to see if chunk type is public
    pub fn is_public(&self) -> bool {
        self.name[1] & BIT_FIVE != BIT_FIVE
    }

    /// Checks third byte to see if reserved bit is valid
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.name[2] & BIT_FIVE != BIT_FIVE
    }

    /// Checks fourth byte to see if chunk type is safe to copy
    pub fn is_safe_to_copy(&self) -> bool {
        self.name[3] & BIT_FIVE == BIT_FIVE
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        ensure!(
            value.iter().all(|b| b.is_ascii_alphabetic()),
            "Chunk Type must be ASCII alphabetic"
        );
        Ok(ChunkType { name: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(
            s.chars().all(|c| c.is_ascii_alphabetic()),
            "Chunk Types must be ASCII alphabetic"
        );
        Ok(ChunkType {
            name: s.as_bytes().try_into()?,
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.name).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
