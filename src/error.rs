use anyhow::Result;
use log::error;
use std::process::exit;

pub fn handle_error<T>(result: Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }
}
