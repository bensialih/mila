use inotify::{EventMask, Inotify, WatchMask};
use std::{path::PathBuf, sync::Mutex};

#[path = "../files.rs"]
mod files;
#[path = "../settings.rs"]
mod settings;

use files::{FileObj, FileTrait};
use log::info;

fn main() {
    env_logger::init();

    let mut file = Mutex::new(PathBuf::from("/tmp/inote.json"));
    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let file_inner = file.get_mut().unwrap();

    assert!(file_inner.exists());

    let mut watches = inotify.watches();
    watches
        .add(&file_inner, WatchMask::MODIFY)
        .expect("failed to watch required file.");

    let mut buffer = [0u8; 4096];

    info!("Watching file size change");
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("failed to read events.");

        for event in events {
            if event.mask.contains(EventMask::MODIFY) {
                let file_inner = file.get_mut().unwrap();
                let size = file_inner.metadata().unwrap().len();

                if size > 50 {
                    let file_obj: FileObj = (*file_inner).clone().into();

                    let count = file_obj.get_highest_count();
                    // rotate files and create an empty original file
                    file_obj.rotate(count, Some(count.unwrap_or(0) + 1));

                    // remove original fd and add new one to notify with same "MODIFY" params
                    watches.remove(event.wd).unwrap();
                    watches.add(&file_inner, WatchMask::MODIFY).unwrap();

                    info!("rotated file");
                }

                info!("file has been modified {:?} --- size: {}", event.name, size);
            }
        }
    }
}
