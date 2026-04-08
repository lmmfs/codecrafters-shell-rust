//use std::fmt::Display
use thiserror::Error;


#[derive(Debug, Error)]
pub enum CustomError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("Command exited with status: {0}")]
    CommandInvalidExitStatus(String),
    #[error("Failed to execute: {0}")]
    CommandExecutionFailed(String)
}



