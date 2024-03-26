mod file;
mod helpers;
mod models;
mod processes;

use helpers::FileObj;
// use processes::{loop_settings, file_listener};
use file::{check_file_size, rotate_file, settings};
use futures::{future, future::BoxFuture, StreamExt};
use models::FileSetting;
use std::sync::Arc;
use std::{path::PathBuf, thread, time::Duration};
use tokio::sync::{mpsc::channel, Mutex};
use tokio::{self, sync::Notify};

use processes::{file_listen, rotate_file_action, Actions, SleepArc, TaskWrapper};

#[tokio::main]
async fn main() {
    let files: FileSetting = FileSetting {
        settings_path: FileObj::new("./data/default_settings.json".to_string()),
        log_path: FileObj::new("./rotate_dir/tmp.txt".to_string()),
    };

    let (sender, receiver) = channel::<Actions>(3);
    let container = TaskWrapper { sender };

    let task = tokio::spawn(file_listen(container, files));
    let receiver_task = tokio::spawn(rotate_file_action(receiver));
    let mut tasks = Vec::new();
    tasks.push(task);
    tasks.push(receiver_task);

    future::join_all(tasks).await;
}
