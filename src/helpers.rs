use std::fmt::format;
use std::fs::rename;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct FileObj {
    pub parent: PathBuf,
    pub file_name: String,
    pub extension: String,
}

impl FileObj {
    pub fn incremented(&self, number: u32) -> String {
        match self.parent.to_str() {
            // if parent is empty- ie relative file, dont state parent
            Some("") => format!("{}.{number}.{}", self.file_name, self.extension,),
            _ => format!(
                "{}/{}.{number}.{}",
                self.parent.to_str().unwrap(),
                self.file_name,
                self.extension,
            ),
        }
    }

    pub fn new(file_location: String) -> Self {
        let filename = String::from(file_location.as_str());
        let file_path = Path::new(&filename).to_owned();

        let parent = file_path.parent().unwrap().to_owned();
        let file_name = file_path.file_stem().unwrap();
        let extension = file_path.extension().unwrap();

        FileObj {
            parent,
            file_name: file_name.to_string_lossy().to_string(),
            extension: extension.to_string_lossy().to_string(),
        }
    }

    pub fn to_pathbuf(&self) -> PathBuf {
        let mut new = self.parent.clone();
        new.push(format!("{}.{}", self.file_name, self.extension));
        return new;
    }
}

impl ToString for FileObj {
    fn to_string(&self) -> String {
        match self.parent.to_str() {
            // if parent is empty- ie relative file, dont state parent
            Some("") => format!("{}.{}", self.file_name, self.extension),
            _ => format!(
                "{}/{}.{}",
                self.parent.to_str().unwrap(),
                self.file_name,
                self.extension,
            ),
        }
    }
}

pub struct TestFile {
    pub files: Vec<String>,
}

impl TestFile {
    pub fn create(&self) {
        for file in self.files.iter() {
            let value = file.as_str();

            Command::new("sh")
                .arg("-c")
                .arg(format!("touch {}", value))
                .status()
                .expect("expected to create file using touch");
        }
    }

    pub fn delete(&self) {
        for file in self.files.iter() {
            let value = file.as_str();
            if File::open(value).is_ok() {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("rm {}", value))
                    .status()
                    .expect("failed to delete existing test file");
            }
        }
    }

    pub fn get_path(&self) -> FileObj {
        FileObj::new(String::from(self.files[0].as_str()))
    }
}

pub fn file_exists(file_location: String) -> bool {
    return Path::new(&file_location).exists();
}

pub fn get_highest_file(mut file_location: FileObj) -> u32 {
    // let original = file_location.to_string();
    let mut counter = 1;

    while file_exists(file_location.incremented(counter)) {
        // File::open(*file_location.incremented(counter)).is_ok() {
        counter += 1;
    }
    return counter;
}

pub fn move_file(from_file: FileObj, to_file: FileObj) -> bool {
    let from_loc: &str = &from_file.to_string();
    let to_loc: &str = &to_file.to_string();
    let result = rename(Path::new(&from_loc), Path::new(&to_loc));
    return result.is_ok();
}

#[cfg(test)]
mod tests {
    use crate::helpers::{file_exists, get_highest_file, move_file, FileObj, TestFile};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_get_highest_file() {
        let mut files: Vec<String> = Vec::new();
        let parent = PathBuf::from_str("./data").unwrap();
        let file_obj = FileObj {
            parent: parent,
            file_name: "t_file".to_string(),
            extension: "json".to_string(),
        };

        files.push(file_obj.to_string());
        files.push(file_obj.incremented(1));
        files.push(file_obj.incremented(2));
        files.push(file_obj.incremented(3));

        let builder = TestFile { files };
        builder.create();

        let counter = get_highest_file(file_obj);
        assert_eq!(counter, 4);
        builder.delete();
    }

    #[test]
    fn test_transfer_file() {
        let parent = PathBuf::from_str("./data").unwrap();
        let mut files_to_delete: Vec<String> = Vec::new();
        let mut file_builder = TestFile {
            files: files_to_delete,
        };

        let file_obj = FileObj {
            parent: parent,
            file_name: "t_file".to_string(),
            extension: "json".to_string(),
        };

        // add base host test file
        let file_from = file_obj.to_string();
        file_builder.files.push(file_from.clone());
        file_builder.create();

        let file_to_transfer = file_obj.incremented(4);

        move_file(
            FileObj::new(file_from.to_string()),
            FileObj::new(file_to_transfer.to_string()),
        );

        // check that from_file is deleted and new file "file_to_transfer" exists
        assert_eq!(false, file_exists(file_from.to_string()));
        assert_eq!(true, file_exists(file_to_transfer.to_string()));

        file_builder.files.push(file_to_transfer.clone());
        file_builder.delete();

        assert_eq!(false, file_exists(file_to_transfer.to_string()));
    }

    #[test]
    fn test_file_path_without_parent() {
        let file_operator = FileObj::new("myfile.txt".to_string());

        assert_eq!(file_operator.to_string(), String::from("myfile.txt"))
    }
}
