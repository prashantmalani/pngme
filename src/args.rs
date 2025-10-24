use std::path::PathBuf;

#[derive(Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
}

#[derive(PartialEq, Debug)]
pub enum ArgErr {
    InvalidCommand(String),
    MissingArgs(String),
}

#[derive(Debug)]
pub struct EncodeArgs {
    file: PathBuf,
    chunk_type: String,
    payload: String,
}

#[derive(Debug)]
pub struct DecodeArgs {
    file: PathBuf,
    chunk_type: String,
}

#[derive(Debug)]
pub struct RemoveArgs {
    file: PathBuf,
    chunk_type: String
}


pub fn generate_args(command: &str, filepath: &str, chunk_type: Option<&str>,
    payload: Option<&str>) -> Result<PngMeArgs, ArgErr> {
    // Check for valid filepath since that's common to everything.
    match command {
        "encode" => {
            if chunk_type.is_none() {
                Err(ArgErr::MissingArgs(String::from("chunk type")))
            } else if payload.is_none() {
                Err(ArgErr::MissingArgs(String::from("payload")))
            } else {
                Ok(PngMeArgs::Encode(EncodeArgs { file: PathBuf::from(filepath),
                                                  chunk_type: String::from(chunk_type.unwrap()),
                                                  payload: String::from(payload.unwrap()) }))
            }
        },
        "decode" => {
            if chunk_type.is_none() {
                Err(ArgErr::MissingArgs(String::from("chunk type")))
            } else {
                Ok(PngMeArgs::Decode (DecodeArgs { file: PathBuf::from(filepath),
                                                chunk_type: String::from(chunk_type.unwrap()) }))
            }
        }
        "remove" => {
            if chunk_type.is_none() {
                Err(ArgErr::MissingArgs(String::from("chunk type")))
            } else {
                Ok(PngMeArgs::Remove (RemoveArgs { file: PathBuf::from(filepath),
                                                chunk_type: String::from(chunk_type.unwrap()) }))
            }
        }
        _ => Err(ArgErr::InvalidCommand(String::from(command)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_encode_valid() {
        let result = generate_args("encode", "./foo.txt", Some("ruSt"), Some("Deadbeef"));
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PngMeArgs::Encode(_)));
    }

    #[test]
    pub fn test_encode_missing_args_chunk_type() {
        let result = generate_args("encode", "./foo.txt", None, Some("Deadbeef"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArgErr::MissingArgs(String::from("chunk type")));
    }

    #[test]
    pub fn test_encode_missing_args_payload() {
        let result = generate_args("encode", "./foo.txt", Some("ruSt"), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArgErr::MissingArgs(String::from("payload")));
    }

    #[test]
    pub fn test_decode_valid() {
        let result = generate_args("decode", "./foo.txt", Some("ruSt"), None);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PngMeArgs::Decode(_)));
    }

    #[test]
    pub fn test_decode_missing_args_chunk_type() {
        let result = generate_args("decode", "./foo.txt", None, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArgErr::MissingArgs(String::from("chunk type")));
    }

    #[test]
    pub fn test_remove_valid() {
        let result = generate_args("remove", "./foo.txt", Some("ruSt"), None);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PngMeArgs::Remove(_)));
    }

    #[test]
    pub fn test_remove_missing_args_chunk_type() {
        let result = generate_args("remove", "./foo.txt", None, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ArgErr::MissingArgs(String::from("chunk type")));
    }
}

