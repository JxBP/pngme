use std::{fmt::Display, str::FromStr};

/// A representation of a PNG 1.2 conform chunk type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType([u8; 4]);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkTypeParseError {
    #[error("expected a length of 4 bytes but got {0} instead")]
    InvalidLength(usize),

    #[error("found an invalid byte {0}, only [A-Za-z] allowed")]
    InvalidByte(u8),
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeParseError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value.iter().find(|b| !b.is_ascii_alphabetic()) {
            Some(b) => Err(ChunkTypeParseError::InvalidByte(*b)),
            None => Ok(Self(value)),
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s
            .as_bytes()
            .try_into()
            .map_err(|_| ChunkTypeParseError::InvalidLength(s.len()))?;
        Self::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

impl ChunkType {
    /// Returns the 4-bytes making up the chunk type
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        nth_bit(self.0[0], 5)
    }

    pub fn is_public(&self) -> bool {
        nth_bit(self.0[1], 5)
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        nth_bit(self.0[2], 5)
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !nth_bit(self.0[3], 5)
    }
}

#[inline]
fn nth_bit(num: u8, n: usize) -> bool {
    num & (1 << n) == 0
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

        let chunk = ChunkType::try_from(b"Ru1t".to_owned());
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
