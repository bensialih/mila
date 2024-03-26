use std::time::Duration;
use crate::models::{FileSetting, Settings};
use std::sync::Arc;
use tokio::sync::Mutex;
use inotify::{Inotify, WatchMask};
use futures_util::StreamExt;
use tokio::sync::mpsc::{Sender, Receiver};

use futures::Future;
use tokio::macros::support::Pin;
use std::path::PathBuf;
use tokio::fs::File;

pub enum Actions {
    Rotate,
    RotatedFromSleep
}

pub struct TaskWrapper {
    pub sender: Sender<Actions>,
}

type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type SleepArc = Arc<Mutex<u64>>;

async fn get_size(path: PathBuf) -> u64 {
    // if this is async then I think the race condition is mitigated
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

pub fn rotate_file_action(mut receiver: Receiver<Actions>) -> AsyncTask {
    return Box::pin(async move{
        loop {
            let event = receiver.try_recv();
            if event.is_err() {
                continue;
            }

            match event {
                Ok(Actions::Rotate) => {
                    println!("rotating file now");
                },
                Ok(Actions::RotatedFromSleep) => {
                    println!("slept great. Now rotating!!");
                }
                _ => {}
            }
        }
    });
}

pub fn file_listen(container: TaskWrapper, files: FileSetting) -> AsyncTask {
    return Box::pin(async move {
        let log_file = files.log_path.to_pathbuf();
        let settings_file = files.settings_path.to_pathbuf();

        assert!(log_file.exists());
        assert!(settings_file.exists());


        let settings: Settings = files.settings_path.clone().into();

        let sleep_couner: SleepArc = Arc::new(Mutex::new( settings.sleep_counter));
        let file_size: SleepArc = Arc::new(Mutex::new(settings.file_size.bytes()));

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
                                    container.sender.send(Actions::Rotate).await.unwrap();
                                    break;
                                } else if inner.wd == settings_handle {
                                    let new_settings: Settings = files.settings_path.clone().into();

                                    let mut inner_sleep = sleep_couner.lock().await;
                                    *inner_sleep = new_settings.sleep_counter;

                                    let mut inner_size = file_size.lock().await;
                                    *inner_size = new_settings.file_size.bytes();

                                    break;
                                } else {
                                    // do nothing
                                }
                            }
                        }
                    }  => {},
                    _ = sleep_and_wait(sleep_couner.clone()) => {
                        container.sender.send(Actions::RotatedFromSleep).await.unwrap();
                        println!("slept and completed for {:?}", sleep_couner);
                    }
                }
            }
        }
    });
}
