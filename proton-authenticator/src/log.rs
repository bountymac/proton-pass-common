use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub trait Logger: Send + Sync {
    fn log(&self, level: LogLevel, msg: String);
}

static LOGGER: RwLock<Option<Arc<dyn Logger>>> = RwLock::new(None);

pub fn register_authenticator_logger(logger: Arc<dyn Logger>) {
    if let Ok(mut r) = LOGGER.write() {
        *r = Some(logger);
    }
}

#[allow(dead_code)]
pub(crate) fn with_logger<F>(cb: F)
where
    F: FnOnce(Arc<dyn Logger>),
{
    if let Ok(ref logger) = LOGGER.read() {
        if let Some(logger) = logger.as_ref() {
            cb(logger.clone());
        }
    }
}

macro_rules! trace {
    ($($args:tt)*) => {
        with_logger(|logger| logger.log(LogLevel::Trace, format_args!($($args)*),))?;
    }
}

macro_rules! debug {
    ($($args:tt)*) => {
        with_logger(|logger| logger.log(LogLevel::Debug, format_args!($($args)*),))?;
    }
}
macro_rules! info {
    ($($args:tt)*) => {
        with_logger(|logger| logger.log(LogLevel::Info, format_args!($($args)*),))?;
    }
}

macro_rules! warn {
    ($($args:tt)*) => {
        with_logger(|logger| logger.log(LogLevel::Warn, format_args!($($args)*),))?;
    }
}
macro_rules! error {
    ($($args:tt)*) => {
        with_logger(|logger| logger.log(LogLevel::Error, format_args!($($args)*),))?;
    }
}
