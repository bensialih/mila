use std::fs::File;
use std::io::BufReader;

// use std::fs::PathBuf;
use crate::{
    file::FileSize,
    helpers::{self, FileObj},
};
use serde::{Deserialize, Serialize};

pub struct FileSetting {
    pub settings_path: helpers::FileObj,
    pub log_path: helpers::FileObj,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub sleep_counter: u64,
    #[serde(flatten)]
    pub file_size: FileSize,
}

// #[derive(Debug)]
// pub enum Actions {
//     UpdateSettings,
//     RotateLogFile,
// }

impl From<FileObj> for Settings {
    fn from(value: FileObj) -> Self {
        let location = value.to_string();
        let file =
            File::open(&location).expect(&format!("failed to find file in location {}", location));
        let buffer = BufReader::new(file);
        serde_json::from_reader(buffer).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_new_settings_format() {
        let file_content = r#"{"sleep_counter": 2, "mb": 1}"#;
        println!("file content is {}", file_content);
        let item: Settings = from_str(file_content).unwrap();

        assert_eq!(item.file_size.bytes(), 1_048_576);
    }

    #[test]
    fn test_settings_reverse_objectify() {
        let item = Settings {
            sleep_counter: 100,
            file_size: FileSize::Mb(1),
        };

        let expected = r#"{"sleep_counter":100,"mb":1}"#;
        assert_eq!(serde_json::to_string(&item).unwrap(), expected);
    }
}
