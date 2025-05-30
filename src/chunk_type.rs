
use std::{io, str::FromStr};

use crate::{Error, Result};

#[derive(Debug)]
pub struct ChunkType {
    data: [u8; 4], // Actual data.
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.data
    }

    fn is_critical(&self) -> bool {
        if u8::is_ascii_uppercase(&self.data[0]) {
            return true
        }

        return false
    }

    fn is_public(&self) -> bool {
        if u8::is_ascii_uppercase(&self.data[1]) {
            return true
        }

        return false
    }

    fn is_reserved_bit_valid(&self) -> bool {
        if u8::is_ascii_uppercase(&self.data[2]) {
            return true
        }

        return false
    }

    fn is_safe_to_copy(&self) -> bool {
        if u8::is_ascii_lowercase(&self.data[3]) {
            return true
        }

        return false
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false
        }

        let orig_str = std::str::from_utf8(&self.data);
        match orig_str {
            Ok(s) => {
                return s.is_ascii()
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return false
            },
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(ChunkType{data: value})
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "wrong_size")))
        }

        if !s.chars().all(char::is_alphabetic) {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput,
                                        "provided string is not ASCII/letters")))
        }

        match s.as_bytes().try_into() {
            Ok(arr) => Ok(ChunkType{data: arr}),
            Err(e) =>  Err(Box::new(e))
        }
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }

    fn ne(&self, other: &Self) -> bool {
        self.bytes() != other.bytes()
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.data).unwrap())
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