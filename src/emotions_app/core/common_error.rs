use std::any::Any;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
pub enum ErrorType {
    IoError,
    ParingError,
}

#[derive(Debug)]
pub struct CommonError {
    error: ErrorType,
    msg: String,
    cause: Option<Box<dyn Error>>,
}

unsafe impl Send for CommonError {}

unsafe impl Sync for CommonError {}

impl CommonError {
    pub fn new(error: ErrorType, msg: Option<String>, cause: Option<Box<dyn Error>>) -> Self {
        let msg = msg.unwrap_or("".to_string());

        Self { error, msg, cause }
    }
}

impl Error for CommonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            Some(e) => Some(e.deref()),
            None => None,
        }
    }
}

impl Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP Error")
    }
}

impl From<reqwest::Error> for CommonError {
    fn from(value: reqwest::Error) -> Self {
        CommonError {
            cause: Some(Box::new(value)),
            error: ErrorType::IoError,
            msg: "".to_string(),
        }
    }
}

impl From<serde_json::Error> for CommonError {
    fn from(value: serde_json::Error) -> Self {
        CommonError {
            cause: Some(Box::new(value)),
            error: ErrorType::ParingError,
            msg: "".to_string(),
        }
    }
}

impl From<std::io::Error> for CommonError {
    fn from(value: std::io::Error) -> Self {
        CommonError {
            cause: Some(Box::new(value)),
            error: ErrorType::IoError,
            msg: "".to_string(),
        }
    }
}
