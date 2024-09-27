use crate::models::error_map::ErrorMap;

pub mod config_actor;
pub mod logger;

pub trait Runnable {
    fn setup(&mut self) -> Result<(), ErrorMap>;
    fn start() -> ErrorMap;
    fn stop();
}
