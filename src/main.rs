mod file;
mod helpers;
use file::{check_file_size, rotate_file, settings};
use std::{thread, time::Duration};

fn main() {
    let settings_file = "data/default_settings.json";
    let file_location = "data/test_file.json";
    let mut file_size: u64 = 120;

    let mut data = settings(Some(settings_file));
    println!("total bytes >>> {}", data.bytes());

    'checker: loop {
        if file_size < data.bytes() {
            data = settings(Some(settings_file));
            println!("current size {file_size:?} --- {}", data.bytes());

            file_size = check_file_size(file_location).unwrap();

            println!("current size after {file_size:?}");

            thread::sleep(Duration::from_millis(100));
        } else {
            rotate_file(String::from(file_location));
            break 'checker;
        }
        // break 'checker;
    }
}
