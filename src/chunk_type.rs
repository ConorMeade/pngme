use std::fmt::Display;
//use std::fmt::Error;
use std::str::{self, FromStr};

//use std::error::Error;
use pngme::{PngError, Result, CHUNK_SIZE};

/// A 4-byte chunk type code for PNG file
/// Each chunk has a type that can be represented as a 4 character string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType(
    /// bytes encoding of chunk type
    [u8; 4],
);

#[allow(dead_code)]
impl ChunkType {
    /// Bytes representation for ChunkType
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    /// The char at certian index of ChunkType's bytes
    fn at_char(&self, index: usize) -> char {
        (self.0)[index] as char
    }

    /// Returns true if the chunk is critical.
    pub fn is_critical(&self) -> bool {
        self.at_char(0).is_uppercase()
    }

    /// Returns true if the chunk is public.
    pub fn is_public(&self) -> bool {
        self.at_char(1).is_uppercase()
    }

    /// Checks whether the reserved bit of the chunk name is set.
    /// If it is set the chunk name is invalid.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.at_char(2).is_uppercase()
    }

    /// Returns true if the chunk is safe to copy if unknown.
    pub fn is_safe_to_copy(&self) -> bool {
        self.at_char(3).is_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_public() && self.is_reserved_bit_valid()
    }
}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngError;

    fn try_from(bytes: [u8; CHUNK_SIZE]) -> Result<Self> {
        if bytes.iter().all(|&c| (c as char).is_ascii_alphabetic()) {
            Ok(Self(bytes))
        } else {
            Err(PngError::Custom("Invalid ascii character, must be alphabetic".to_owned()))
        }
    }
}

impl FromStr for ChunkType { 
    type Err = PngError;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() <= 0 {
            //Err(FromStrError::Empty)
            Err(PngError::Custom("Empty String".to_owned()))
        } else if s.len() != 4 {
            Err(PngError::Custom("Not proper CHUNK_SIZE length".to_owned()))
        } else if !s.chars().all(|c: char| c.is_ascii_alphabetic()) {
            Err(PngError::Custom(
                "Invalid ascii character, must be alphabetic".to_owned(),
            ))
        } else {
            let mut chunk_as_bytes: [u8; 4] = [0; 4];
            chunk_as_bytes.clone_from_slice(s.as_bytes());
            Ok(ChunkType(chunk_as_bytes))
        }
        
    }
}


impl Display for ChunkType {
    /// display the [`ChunkType`] using its string representation of bytes
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}", 
            str::from_utf8(&self.bytes()).map_err(PngError::from)?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected: [u8; 4] = [82, 117, 83, 116];
        let actual: ChunkType = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected: ChunkType = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        println!("{}", expected);
        let actual: ChunkType = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
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
        println!("{}", chunk);
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
        println!("{}", chunk);
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
