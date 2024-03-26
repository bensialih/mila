mod file;
mod helpers;
mod processes;
mod models;

use helpers::FileObj;
// use processes::{loop_settings, file_listener};
use file::{check_file_size, rotate_file, settings};
use std::{path::PathBuf, thread, time::Duration};
use futures::{future, StreamExt, future::BoxFuture};
use tokio::{self, sync::Notify};
use models::FileSetting;
use tokio::sync::{Mutex, mpsc::channel};
use std::sync::Arc;


use processes::{
    TaskWrapper, SleepArc, Actions,
    file_listen, rotate_file_action
};

#[tokio::main]
async fn main() {
    let files: FileSetting = FileSetting {
        settings_path: FileObj::new("./data/default_settings.json".to_string()),
        log_path: FileObj::new("./rotate_dir/tmp.txt".to_string()),
    };

    let (sender, receiver) = channel::<Actions>(3);
    let container = TaskWrapper{sender};


    // // may not be necessary as settings file is passed in FileSettings
    // let counter: SleepArc = Arc::new(Mutex::new(2));
    // // same here. should be as part of FileSettings
    // let desired_file_size = Arc::new(Mutex::new(200));

    let task = tokio::spawn(file_listen(container, files));
    let receiver_task = tokio::spawn(rotate_file_action(receiver));
    let mut tasks = Vec::new();
    tasks.push(task);
    tasks.push(receiver_task);

    future::join_all(tasks).await;
}
