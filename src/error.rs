use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppError {
    InvalidParameter,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidParameter => f.write_str("the parameter passed is invalid"),
        }
    }
}

impl std::error::Error for AppError {}
