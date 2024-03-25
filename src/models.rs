use tokio::sync::Notify;

// use std::fs::PathBuf;
use crate::helpers;

pub struct FileSetting {
    pub settings_path: helpers::FileObj,
    pub log_path: helpers::FileObj
}

pub struct Settings {
    pub sleep_count: u64,
    pub notify: Notify,
}

#[derive(Debug)]
pub enum Actions {
    UpdateSettings,
    RotateLogFile,
}