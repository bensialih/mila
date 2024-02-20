
use std::process::Command;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct TestFile {
    pub files: Vec<Box<String>>,
}

pub struct FileObj {
    pub parent: Box<PathBuf>,
    pub file_name: Box<String>,
    pub extension: Box<String>,
}

impl FileObj {
    pub fn incremented(&self, number: u32) -> Box<String> {
        Box::new(format!("{}/{}.{number}.{}",
            self.parent.to_str().unwrap(),
            self.file_name,
            self.extension,
        ))
    }

    pub fn to_string(&self) -> Box<String> {
        Box::new(format!("{}/{}.{}", 
            self.parent.to_str().unwrap(),
            self.file_name,
            self.extension,
        ))
    }
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
        let filename = String::from(self.files[0].as_str());
        let file_path = Path::new(&filename).to_owned();

        let parent = file_path.parent().unwrap().to_owned();
        let file_name = file_path.file_name().unwrap();
        let extension = file_path.extension().unwrap();

        FileObj {
            parent: Box::new(parent),
            file_name: Box::new(String::from(file_name.to_str().unwrap())),
            extension: Box::new(String::from(extension.to_str().unwrap())),
        }
    }
}


pub fn file_exists(file_location: &str) -> bool {
    return Path::new(file_location).exists();
}