use std::fs;

use fern::{Dispatch, InitError};

use crate::models::error_map::ErrorMap;

use super::Runnable;

pub struct Logger {
    pub log_folder: String,
    pub general_log: Dispatch,
    pub error_log: Dispatch,
}

impl Logger {
    pub fn new(log_path: String) -> Self {
        Self {
            log_folder: log_path,
            general_log: Dispatch::new(),
            error_log: Dispatch::new(),
        }
    }
}

impl Runnable for Logger {
    fn setup(&mut self) -> Result<(), ErrorMap> {}
    fn start() -> ErrorMap {}
    fn stop() {}
}
