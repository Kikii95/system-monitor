use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("Failed to initialize system info: {0}")]
    SystemInit(String),

    #[error("Failed to collect metrics: {0}")]
    Collection(String),

    #[error("GPU not available or not supported")]
    GpuNotAvailable,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
