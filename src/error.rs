use aws_sdk_ec2::error::SdkError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EzsshError {
    #[error("config parse error {0}")]
    ConfigParseError(String),
    #[error("command parse error {0}")]
    CommandParseError(String),
    #[error("IO Error {0}")]
    IOError(#[from] std::io::Error),
    #[error("AWS SDK Error {0}")]
    AWSSDKError(String),
}

pub type Result<T> = std::result::Result<T, EzsshError>;

impl<E> From<SdkError<E>> for EzsshError {
    fn from(value: SdkError<E>) -> Self {
        EzsshError::AWSSDKError(value.to_string())
    }
}
