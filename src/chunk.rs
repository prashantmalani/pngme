
use crate::{chunk_type::ChunkType, Error, Result};
use core::str;
use std::io;
use std::str::FromStr;

use crc32fast::Hasher;

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {chunk_type: chunk_type, data: data}
    }

    pub fn length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    fn crc(&self) -> u32 {
        let mut crc_payload : Vec<u8> =  Vec::new();
        crc_payload.extend(self.chunk_type.bytes());
        crc_payload.extend(self.data.as_slice());

        //let crc_cal = Crc::<u32>::new(&CRC_32_CKSUM);
        //return crc_cal.checksum(crc_payload.as_slice())
        let mut hasher = Hasher::new();
        hasher.update(crc_payload.as_slice());
        return hasher.finalize()
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data_as_string(&self) -> Result<String> {
        if self.chunk_type.is_valid() {
            Ok(String::from_utf8(self.data.as_slice().to_vec()).unwrap())
        } else {
            Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "invalid chunk")))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_data: Vec<u8> = (self.data.len() as u32)
        .to_be_bytes()
        .iter()
        .chain(self.chunk_type.bytes().iter())
        .chain(self.data.as_slice().iter())
        .chain(self.crc().to_be_bytes().iter())
        .copied()
        .collect();

        return chunk_data;
    }
 }

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        // A valid chunk needs:
        // Length: 4 bytes
        // Type: 4 bytes
        // CRC: 4 bytes
        // So any data slice less than 12 bytes can be eliminated immediately.
        if value.len() < 12 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "invalid length")));
        }
        // Extract Chunk Type and check it.
        let type_bytes = &value[4..8];
        let ctype = match ChunkType::from_str(str::from_utf8(type_bytes).unwrap()) {
            Ok(c) => {
                if !c.is_valid() {
                    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "invalid chunk type")));
                }
                c
            },
            Err(e) => return Err(e),
        };

        let length_field = u32::from_be_bytes(value[..4].try_into().unwrap()) as usize;
        let data = &value[8..(8 + length_field)];
        let chunk = Chunk{chunk_type: ctype, data: data.to_vec()};

        // Make sure the provided CRC matches what we calculate.
        if chunk.crc() != u32::from_be_bytes(value[(8 + length_field)..(12 + length_field)].try_into().unwrap()) {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "invalid CRC provided")));
        }

        Ok(chunk)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

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