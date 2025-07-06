#[derive(Debug)]
pub enum FugaError {
    ConfigError(confy::ConfyError),
    ConfigPathMissing,
    IoError(std::io::Error),
    FileNotFound(String),
    PermissionError { path: String, message: String },
    OperationFailed(String),
    DuplicatePath { source: String, destination: String },
    FileSystemError(String),
}

impl std::fmt::Display for FugaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FugaError::ConfigError(e) => write!(f, "Configuration error: {e}"),
            FugaError::ConfigPathMissing => write!(f, "Configuration path is missing"),
            FugaError::IoError(e) => write!(f, "IO error: {e}"),
            FugaError::FileNotFound(path) => write!(f, "File not found: {path}"),
            FugaError::PermissionError { path, message } => {
                write!(f, "Permission denied for {path}: {message}")
            }
            FugaError::OperationFailed(msg) => write!(f, "Operation failed: {msg}"),
            FugaError::DuplicatePath {
                source,
                destination,
            } => write!(
                f,
                "Source and destination are the same: {source} -> {destination}"
            ),
            FugaError::FileSystemError(msg) => write!(f, "File system error: {msg}"),
        }
    }
}

impl std::error::Error for FugaError {}

impl From<confy::ConfyError> for FugaError {
    fn from(err: confy::ConfyError) -> Self {
        FugaError::ConfigError(err)
    }
}

impl From<std::io::Error> for FugaError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => FugaError::FileNotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => FugaError::PermissionError {
                path: "unknown".to_string(),
                message: err.to_string(),
            },
            _ => FugaError::IoError(err),
        }
    }
}

pub type FugaResult<T> = Result<T, FugaError>;
