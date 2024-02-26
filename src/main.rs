#![allow(unused)]
use std::{fs::{create_dir, create_dir_all, remove_dir_all, rename, DirBuilder, File}, iter, path::{Path, PathBuf}, sync::Mutex};
use std::rc::Rc;

#[derive(Clone)]
struct FileObj {
    parent: PathBuf,
    file_stem: String,
    suffix: String,
}

impl FileObj {
    fn new(path: &PathBuf) -> Self {
        FileObj {
            parent: path.parent().unwrap().to_path_buf(),
            file_stem: path.file_stem().unwrap().to_string_lossy().to_string(),
            suffix: path.extension().unwrap().to_string_lossy().to_string(),
        }
    }
}

struct FolderOperator {
    files: Mutex<Vec<FileObj>>,
    folder: Option<PathBuf>,
}

trait FolderTrait {
    fn add_file(self, file_name: &str, auto_create: Option<bool>) -> Self;
    fn with_directory(self, folder_name: &'static str) -> Self;
    fn new() -> Self;
    fn delete(self);
}

trait FileTrait {
    fn touch(&self);
    fn delete(&self);
    fn move_file(&self, new_location: Self);

    fn get_highest_count(&self) -> Option<u32>;
    fn rotate(&self, from_number: Option<u32>, to_number: Option<u32>);
}

impl FileTrait for FileObj {
    fn touch(&self) {
        let file_loc = format!("{}.{}", self.file_stem, self.suffix);
        let mut full_path = self.parent.clone();
        full_path.push(file_loc);
        File::create(full_path).unwrap();
    }

    fn delete(&self){todo!()}
    fn move_file(&self, new_location: Self){todo!()}

    /// gets either a number or None if there is no other files that have been incremented
    fn get_highest_count(&self) -> Option<u32> {
        
        let top_count = 1;

        for count in 1..100_000 {
            let mut parent = self.parent.clone();
            parent.push(format!("{}.{}.{}", self.file_stem, count, self.suffix));

            if Path::new(&parent).exists() == false {
                if count == 1 {
                    return None;
                }
                else {
                    return Some(count - 1);
                }
            }
        }
        None
    }

    fn rotate(&self, from_number: Option<u32>, to_number: Option<u32>){
        // let count = self.get_highest_count();
        if from_number.is_none() {
            // means there is no files above the original so move file from file.txt to file.1.txt
            let mut original = self.parent.clone();
            original.push(format!("{}.{}", self.file_stem, self.suffix));

            let mut to_file = self.parent.clone();
            to_file.push(format!("{}.1.{}", self.file_stem, self.suffix));

            rename(original.clone(), to_file).unwrap();
            File::create(original);
        } else {
            // these number must be 1 or above
            let mut from_num = from_number.unwrap();
            let mut to_num = to_number.unwrap();
            let org_path_str = format!("{}.{}.{}", self.file_stem, from_num, self.suffix);
            let to_path_str = format!("{}.{}.{}", self.file_stem, to_num, self.suffix);

            // from path location
            let mut from_path = self.parent.clone();
            from_path.push(org_path_str.clone());

            // to file location
            let mut to_file = self.parent.clone();
            to_file.push(to_path_str);

            // move file to new location
            rename(from_path, to_file).unwrap();

            to_num = from_num;
            from_num -= 1;

            if from_num == 0 {
                self.rotate(None, Some(to_num));
            } else {
                self.rotate(Some(from_num), Some(to_num));
            }
        }

    }
}

impl FolderTrait for FolderOperator {
    fn add_file(mut self, file_name: &str, auto_create: Option<bool>) -> Self {
        let path = PathBuf::from(file_name);
        let name = path.file_stem().unwrap();
        let suffix = path.extension().unwrap();

        let mut parent_path = self.folder.clone().unwrap();
        let file = FileObj {
            parent: parent_path.clone(),
            file_stem: String::from(name.to_str().unwrap()),
            suffix: String::from(suffix.to_str().unwrap())
        };
        self.files.get_mut().unwrap().push(file.clone());

        if auto_create.unwrap_or(false) {
            file.touch();
        }
        self
    }

    fn with_directory(mut self, dir: &'static str) -> Self {
        if dir.starts_with("./") == false {
            panic!("folder should start with ./. Make sure that folder is relative.")
        }

        let directory = PathBuf::from(dir);
        create_dir(&directory)
            .expect("new folder to be created");

        self.folder = Some(directory);
        return self
    }

    fn new() -> FolderOperator {
        FolderOperator {
            files: Mutex::from(Vec::new()),
            folder: None,
        }
    }

    fn delete(mut self) {
        // deletes folder pathbuf
        let folder = self.folder.as_mut().unwrap();
        if folder.starts_with("./") == false {
            panic!("folder should start with ./ to make sure it is relative")
        }

        let result = remove_dir_all(folder);
        if result.is_ok() {
            self.folder = None;
        } else {
            panic!("failed to create folder");
        }
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_file() {

    }

    #[test]
    fn test_delete_file() {

    }

    #[test]
    fn test_move_file(){

    }



    #[test]
    fn test_folder_creation() {
        let mut builder = FolderOperator::new();
        assert_eq!(builder.files.get_mut().unwrap().len(), 0);
        assert_eq!(builder.folder, None);
    }

    #[test]
    fn test_add_folder_and_delete() {
        let folder_to_create = "./exo_repo/";
        let builder = FolderOperator::new().with_directory(folder_to_create);
        assert_eq!(PathBuf::from(folder_to_create).is_dir(), true);
        builder.delete();
        assert_eq!(PathBuf::from(folder_to_create).is_dir(), false);
    }

    #[test]
    fn test_add_file_touch() {
        let folder = "./exo_repo1";
        let mut paths = PathBuf::from(folder);

        let builder = FolderOperator::new()
            .with_directory(folder)
            .add_file("file.txt", Some(true))
            .add_file("file2.txt", Some(true));

        assert_eq!(File::open("./exo_repo1/file.txt").is_ok(), true);
        assert_eq!(File::open("./exo_repo1/file2.txt").is_ok(), true);
        builder.delete();
    }


    #[test]
    fn test_highest_file_number() {
        let path = PathBuf::from("./exo_folder/file.txt");
        let mut builder = FolderOperator::new()
            .with_directory("./exo_folder")
            .add_file("file.txt", Some(true));

        assert_eq!(Path::new(&path).exists(), true);

        let route_file = FileObj::new(&path);
        assert_eq!(route_file.get_highest_count(), None);

        builder = builder.add_file("file.1.txt", Some(true));

        assert_eq!(route_file.get_highest_count(), Some(1));
        builder.delete();
    }

    #[test]
    fn test_move_and_rotate() {
        let path = PathBuf::from("./exo_folder2/file.txt");
        let path2 = PathBuf::from("./exo_folder2/file.1.txt");
        let path_moved = PathBuf::from("./exo_folder2/file.2.txt");
        let mut builder = FolderOperator::new()
            .with_directory("./exo_folder2")
            .add_file("file.txt", Some(true))
            .add_file("file.1.txt", Some(true));

        let file = FileObj::new(&path);
        let count = file.get_highest_count();

        // rotate files and create an emplty original file
        file.rotate(count, Some(count.unwrap()+1));
        assert_eq!(Path::new(&path_moved).exists(), true);
        assert_eq!(Path::new(&path2).exists(), true);
        assert_eq!(Path::new(&path).exists(), true);

        builder.delete();
    }
}