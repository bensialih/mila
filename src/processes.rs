use std::time::Duration;
use crate::models::{FileSetting};
use std::sync::Arc;
use tokio::sync::Mutex;
use inotify::{Inotify, WatchMask};
use futures_util::StreamExt;
use tokio::sync::mpsc::{Sender, Receiver};

use futures::Future;
use tokio::macros::support::Pin;
use std::path::PathBuf;
use tokio::fs::File;

pub enum Acitons {
    Rotate,
    RotatedFromSleep
}

pub struct TaskWrapper {
    pub sender: Sender<Acitons>,
}

type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type SleepArc = Arc<Mutex<u64>>;

async fn get_size(path: PathBuf) -> u64 {
    todo!("potential race condition here!!");

    let file = File::open(path).await.unwrap();
    let meta = file.metadata().await.unwrap().len();
    return meta;
}

fn sleep_and_wait(sleep_value: Arc<Mutex<u64>>) -> AsyncTask {
    return Box::pin(async move {
        let cloned = sleep_value.clone();
        let value = *cloned.lock().await;
        tokio::time::sleep(Duration::from_secs(value)).await;
        ()
    });
}

pub fn rotate_file_action(mut receiver: Receiver<Acitons>) -> AsyncTask {
    return Box::pin(async move{
        loop {
            let event = receiver.try_recv();
            if event.is_err() {
                continue;
            }

            match event {
                Ok(Acitons::Rotate) => {
                    println!("rotating file now");
                },
                _ => {}
            }
        }
    });
}

pub fn file_listen(container: TaskWrapper, sleep_couner: SleepArc, file_size: SleepArc, files: FileSetting) -> AsyncTask {
    return Box::pin(async move {
        let log_file = files.log_path.to_pathbuf();
        let settings_file = files.settings_path.to_pathbuf();

        assert!(log_file.exists());
        assert!(settings_file.exists());

        let inote = Inotify::init().expect("failed to init inotify");

        let mut watches = inote.watches();
        let log_handle = watches
            .add(log_file.clone(), WatchMask::MODIFY)
            .expect("failed to find log file.");

        let settings_handle = watches
            .add(settings_file, WatchMask::MODIFY)
            .expect("failed to find log file.");

        let mut buffer = [0;1024];
        let mut stream = inote.into_event_stream(buffer).unwrap();

        'outer: loop {
            'inner: loop {
                tokio::select! {
                    _ = async {
                        loop {
                            let file_event = stream.next().await;
                            if let Some(event) = file_event {
                                let inner = event.unwrap();

                                if inner.wd == log_handle{
                                    let size = get_size(log_file.clone()).await;
                                    let desired_size = *file_size.lock().await;

                                    if size < desired_size {
                                        continue;
                                    }

                                    // if size >= desired_size {
                                    println!("file size changed and rotated");
                                    container.sender.send(Acitons::Rotate).await.unwrap();
                                    break;
                                } else if inner.wd == settings_handle {
                                    println!("settings file has changed. need to restart process");
                                    break;
                                } else {
                                    // do nothing
                                }
                            }
                        }
                    }  => {},
                    _ = sleep_and_wait(sleep_couner.clone()) => {
                        container.sender.send(Acitons::RotatedFromSleep).await.unwrap();
                        println!("slept and completed");
                    }
                }
            }
        }
    });
}
