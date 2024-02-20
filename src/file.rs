use regex::Regex;
use serde::Deserialize;
use std::borrow::BorrowMut;
use std::env;
use std::fs::rename;
use std::io::Read;
use std::{fs::File, io, io::Error};
use std::rc::Rc;
use std::process::Command;
use std::path::Path;

use crate::helpers::{FileObj, file_exists};

// use crate::::TestFile;

#[derive(Debug)]
pub enum FileError {
    NotFound,
    FileSizeInvalid,
}

fn rotate(file_location: String, top_number: u32) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    for i in (1..=top_number).rev() {
        let from_file = format!("{}.{}", file_location, i);
        let file_name = format!("{}.{}", file_location, (i+1));

        // if i == 1 {
        //     println!("transfer orriginal file {file_location} to {from_file}");
        // } else {
            // println!("transfer {from_file} to {file_name}");
        // }
        files.push(file_name);
    }

    // move original to .1

    files
}

fn get_highest_file(mut file_location: FileObj) -> u32 {
    // let original = file_location.to_string();
    let mut counter = 1;

    while File::open(*file_location.incremented(counter)).is_ok() {
        counter += 1;
    }
    return counter;
}

pub fn rotate_file(mut file_location: String) {
    // file_location is the base file location
    // 1) get highest number of file number
    // 2) rotate all files into next one up

    
    // let mut file_exist_bool = file_exist(file_location.to_string());

    let mut counter = 1;
    // 'check: loop {
    //     // loop and get the highest non existing file
    //     file_location = format!("{}.{}", "original", counter);
    //     file_exist_bool = file_exist(file_location.to_string());
    //     if file_exist_bool {
    //         break 'check;
    //     }
    //     counter += 1;
    // }

    // print!("max file not found is : {counter}");


    // 'check: loop {
    //     let file_exist = file_exist(file_location.to_string());
    //     if file_exist {
    //         file_location = format!("{}.{}", original, counter.unwrap_or(1));

    //         counter = if counter.is_none() {
    //              Some(1)
    //         } else {
    //             let value = counter.unwrap();
    //             Some(value+1)
    //         }
    //     } else {
    //         println!("current file is {}", file_location);
    //         println!("current counter {}", counter.unwrap_or(0));
    //         rotate(file_location, counter.unwrap_or(0));
    //         break 'check;
    //     }
    // }

    // while  {
    //     let location = file_location.borrow_mut();
    //     // file_location = format!("{}.{}", original, counter);
    //     counter += 1;
    // }
}

fn move_file(from_file: Box<String>, to_file: Box<String>) -> bool {
    let result = rename(Path::new(&from_file.to_string()), Path::new(&to_file.to_string()));
    return result.is_ok();
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

#[derive(Debug, Deserialize)]
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
    use std::str::FromStr;
    use std::path::PathBuf;
    use super::*;
    use crate::helpers::{TestFile, FileObj};
    

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
    fn test_file_rotation() {
        let list = rotate("tmp.txt".to_string(), 5);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_get_highest_file() {
        let mut files: Vec<Box<String>> = Vec::new();
        let parent = PathBuf::from_str("./data").unwrap();
        let file_obj = FileObj{
            parent: Box::new(parent),
            file_name: Box::new("t_file".to_string()),
            extension: Box::new("json".to_string())
        };

        files.push(Box::new(*file_obj.to_string()));
        files.push(Box::new(*file_obj.incremented(1)));
        files.push(Box::new(*file_obj.incremented(2)));
        files.push(Box::new(*file_obj.incremented(3)));

        let builder = TestFile{files};
        builder.create();
        
        let counter = get_highest_file(file_obj);
        assert_eq!(counter, 4);
        builder.delete();
    }

    #[test]
    fn test_file_object_filepath() {
        let parent = PathBuf::from_str("./data").unwrap();
        let file_obj = FileObj{
            parent: Box::new(parent),
            file_name: Box::new("test_file".to_string()),
            extension: Box::new("json".to_string())
        };

        assert_eq!(file_obj.incremented(4), Box::new("./data/test_file.4.json".to_string()));
        assert_eq!(file_obj.to_string(), Box::new("./data/test_file.json".to_string()));
    }

    #[test]
    fn test_transfer_file() {
        let parent = PathBuf::from_str("./data").unwrap();
        let mut files_to_delete: Vec<Box<String>> = Vec::new();
        let mut file_builder = TestFile{files: files_to_delete};


        let file_obj = FileObj{
            parent: Box::new(parent),
            file_name: Box::new("t_file".to_string()),
            extension: Box::new("json".to_string())
        };

        // add base host test file
        let file_from = file_obj.to_string();
        file_builder.files.push(file_from.clone());
        file_builder.create();


        let file_to_transfer = file_obj.incremented(4);

        move_file(
            file_from.clone(), 
            file_to_transfer.to_owned()
        );

        // check that from_file is deleted and new file "file_to_transfer" exists
        assert_eq!(false, file_exists(&file_from));
        assert_eq!(true, file_exists(&file_to_transfer));

        file_builder.files.push(file_to_transfer.clone());
        file_builder.delete();

        assert_eq!(false, file_exists(&file_to_transfer));
    }

}
