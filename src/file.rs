use crate::helpers::FileObj;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{env, fs::File};

#[derive(Debug)]
pub enum FileError {
    NotFound,
    FileSizeInvalid,
}

fn rotate_files(file_location: FileObj, top_number: u32) -> Vec<FileObj> {
    let mut files: Vec<FileObj> = Vec::new();

    for i in (1..=top_number).rev() {
        let file_name = file_location.incremented(i);
        files.push(FileObj::new(file_name.to_string()));
    }
    files
}

pub fn rotate_file(mut file_location: String) {
    // file_location is the base file location
    // 1) get highest number of file number
    // 2) rotate all files into next one up
}

pub fn check_file_size(file_location: &str) -> Result<u64, FileError> {
    let _file = File::open(file_location);
    if let Ok(file) = _file {
        let meta = file.metadata().unwrap();
        Ok(meta.len())
    } else {
        Err(FileError::NotFound)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSize {
    Mb(u64),
    Kb(u64),
    Bytes(u64),
}

impl Default for FileSize {
    fn default() -> Self {
        let envar_size: String = env::var("FILE_SIZE").unwrap_or("1mb".to_string());
        let re = Regex::new(r"(?P<number>\d+)(?P<_type>[A-Za-z]{2})").unwrap();

        re.captures(&envar_size)
            .and_then(|value| {
                let _size = value.name("number").unwrap().as_str();
                let _type = value.name("_type").unwrap().as_str();

                let size: u64 = _size.parse().unwrap();

                match _type {
                    "mb" => Some(FileSize::Mb(size)),
                    "kb" => Some(FileSize::Kb(size)),
                    _ => Some(FileSize::Bytes(size)),
                }
            })
            .unwrap()
    }
}

impl FileSize {
    pub fn bytes(&self) -> u64 {
        const BYTES: u64 = 1024;
        match self {
            Self::Mb(x) => BYTES * BYTES * x,
            Self::Kb(x) => BYTES * x,
            Self::Bytes(x) => *x,
        }
    }
}

pub fn settings(file_location: Option<&str>) -> FileSize {
    todo!("deprecated - using casting from FileObj to Settings");

    let mut file_size: Option<FileSize> = None;

    if let Some(file_location) = file_location {
        let mut file = File::open(file_location).unwrap();
        let mut data: String = String::new();
        file.read_to_string(&mut data);
        file_size = Some(serde_json::from_str::<FileSize>(&data).unwrap());
    }
    file_size.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::file::{rotate_files, FileSize};
    use crate::helpers::FileObj;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_deserializer_lowercase() {
        let settings = serde_json::from_str::<FileSize>(r#"{"kb": 512}"#);
        assert!(settings.is_ok());
    }

    #[test]
    fn test_deserializer_uppercase_fail() {
        assert!(serde_json::from_str::<FileSize>(r#"{"Kb": 512}"#).is_err());
    }

    #[test]
    fn test_file_rotation_relative() {
        let file_operator = FileObj::new("./tmp.txt".to_string());
        let list = rotate_files(file_operator.clone(), 5);
        assert_eq!(list.len(), 5);
        assert_eq!(list[0].to_string(), String::from("./tmp.5.txt"));
    }

    #[test]
    fn test_file_rotation_absolute() {
        let file_operator = FileObj::new("/tmp/tmp.txt".to_string());
        let list = rotate_files(file_operator.clone(), 5);
        assert_eq!(list.len(), 5);
        assert_eq!(list[0].to_string(), String::from("/tmp/tmp.5.txt"));
    }

    #[test]
    fn test_file_object_filepath() {
        let parent = PathBuf::from_str("./data").unwrap();
        let file_obj = FileObj {
            parent: parent,
            file_name: "test_file".to_string(),
            extension: "json".to_string(),
        };

        assert_eq!(
            file_obj.incremented(4),
            "./data/test_file.4.json".to_string()
        );
        assert_eq!(file_obj.to_string(), "./data/test_file.json".to_string());
    }
}
