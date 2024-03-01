use inotify::{EventMask, Inotify, WatchMask};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

#[path = "../files.rs"]
mod files;
#[path = "../settings.rs"]
mod settings;

use files::{FileObj, FileTrait};
use log::info;
use settings::FileSize;
use std::env;

struct FileWatch {
    location: PathBuf,
}

impl FileWatch {
    fn new(path: PathBuf) -> Self {
        FileWatch { location: path }
    }

    fn get_size(&self) -> u64 {
        assert!(self.location.exists());
        FileSize::from(FileObj::from(self.location.clone())).into()
    }

    fn get_file_size(&self) -> u64 {
        self.location.metadata().unwrap().len()
    }
}

fn main() {
    env_logger::init();
    let file_location = env::var("LOG_FILE").expect("no log file stated ");
    let settings_location =
        env::var("SETTINGS_FILE").expect("expected file location for settings used.");

    let mut log_loc = Mutex::new(FileWatch::new(PathBuf::from(file_location).to_owned()));
    let mut setting_loc = Mutex::new(FileWatch::new(PathBuf::from(settings_location).to_owned()));

    let mut size_desired: u64 = setting_loc.get_mut().unwrap().get_size();

    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let mut watches = inotify.watches();

    watches
        .add(&log_loc.get_mut().unwrap().location, WatchMask::MODIFY)
        .expect("failed to watch required file.");

    let settings_fd = watches
        .add(&setting_loc.get_mut().unwrap().location, WatchMask::MODIFY)
        .expect("failed to load settings file");

    let mut buffer = [0u8; 4096];
    info!("Watching file size change");

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("failed to read events.");

        for event in events {
            if event.mask.contains(EventMask::MODIFY) {
                if event.wd == settings_fd {
                    size_desired = setting_loc.get_mut().unwrap().get_size();
                }

                let size: u64 = log_loc.get_mut().unwrap().get_file_size();

                if size > size_desired {
                    let file_location = &log_loc.get_mut().unwrap().location;
                    let file_obj: FileObj = file_location.clone().into();

                    let count = file_obj.get_highest_count();
                    // rotate files and create an empty original file
                    file_obj.rotate(count, Some(count.unwrap_or(0) + 1));

                    // remove original fd and add new one to notify with same "MODIFY" params
                    watches.remove(event.wd).unwrap();
                    watches
                        .add(&log_loc.get_mut().unwrap().location, WatchMask::MODIFY)
                        .unwrap();
                    info!("rotated file");
                }

                info!("file has been modified {:?} --- size: {}", event.name, size);
            }
        }
    }
}
