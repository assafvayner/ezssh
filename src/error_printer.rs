use std::fmt::{Debug, Display};

// Snagged and modified from https://github.com/xetdata/xet-core/blob/2ba9674be84616f18b6c0f0b425bd37391fed251/rust/gitxetcore/src/log.rs#L22-L48
pub trait ErrorPrinter {
    fn log_error<M: Display>(self, message: M) -> Self;
}

impl<T, E: Debug> ErrorPrinter for Result<T, E> {
    /// If self is an Err(e), prints out the given string to tracing::error,
    /// appending "error: {e}" to the end of the message.
    fn log_error<M: Display>(self, message: M) -> Self {
        match &self {
            Ok(_) => {}
            Err(e) => eprintln!("{}, error: {:?}", message, e),
        }
        self
    }
}
