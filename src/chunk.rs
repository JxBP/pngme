use crate::chunk_type::{ChunkType, ChunkTypeParseError};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .cloned()
            .collect();

        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: crc.checksum(&crc_bytes),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkParseError {
    #[error("chunk did not contain all the required data")]
    Incomplete,

    #[error("invalid length field (expected {expected:?}, found {found:?})")]
    InvalidLengthField { expected: u32, found: u32 },

    #[error(transparent)]
    InvalidChunkType(#[from] ChunkTypeParseError),

    #[error("parsed checksum didn't match calculated checksum")]
    InvalidChecksum,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // length (4) + type (4) + data (0) + crc (4) => 12
        // 12 is the smallest chunk that can exist. By checking the length
        // beforehand we can ensure that there will be no panics.
        if value.len() < 12 {
            return Err(ChunkParseError::Incomplete);
        }

        let length = u32::from_be_bytes(value[..4].try_into().unwrap());

        // The size of all fields except `data` are 12 bytes in total.
        if length != value.len() as u32 - 12 {
            return Err(ChunkParseError::InvalidLengthField {
                expected: value.len() as u32 - 12,
                found: length,
            });
        }

        let chunk_type: [u8; 4] = value[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let data = Vec::from(&value[8..value.len() - 4]);
        let crc = u32::from_be_bytes(value[value.len() - 4..].try_into().unwrap());

        let chunk = Self::new(chunk_type, data);
        if crc != chunk.crc() {
            return Err(ChunkParseError::InvalidChecksum);
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ length: {:4}, type: {}, data: {}, crc: {:10} }}",
            self.length,
            self.chunk_type,
            self.data_as_string()
                .unwrap_or_else(|_| "<Invalid UTF-8>".to_owned()),
            self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_invalid_chunk_length() {
        let data_length: u32 = 9999; // Way too big
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert_eq!(
            chunk.err().unwrap(),
            ChunkParseError::InvalidLengthField {
                expected: 42,
                found: 9999
            }
        );
    }

    #[test]
    fn test_empty_chunk() {
        let data: &[u8] = &[];
        let chunk = Chunk::try_from(data);
        assert_eq!(chunk.err().unwrap(), ChunkParseError::Incomplete);
    }

    // TODO: Add 2 tests for invalid checksum and chunk type

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
