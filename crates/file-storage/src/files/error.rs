#[derive(Debug, thiserror::Error)]
#[error("file storage operation failed")]
pub struct FileError {
    #[source]
    source: FileErrorSource,
}

#[derive(Debug, thiserror::Error)]
enum FileErrorSource {
    #[error("file database operation failed")]
    Database(#[source] sqlx::Error),
    #[error("file system operation failed")]
    Io(#[source] std::io::Error),
}

impl From<sqlx::Error> for FileError {
    fn from(source: sqlx::Error) -> Self {
        Self {
            source: FileErrorSource::Database(source),
        }
    }
}

impl From<std::io::Error> for FileError {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: FileErrorSource::Io(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::FileError;

    #[test]
    fn adapter_failure_keeps_a_stable_capability_message_and_source() {
        let error = FileError::from(std::io::Error::other("disk detail"));

        assert_eq!(error.to_string(), "file storage operation failed");
        let kind = error
            .source()
            .expect("capability error should keep its kind");
        let source = kind
            .source()
            .expect("file error should keep its I/O source");
        let source = source
            .downcast_ref::<std::io::Error>()
            .expect("source should remain an I/O error");
        assert_eq!(source.to_string(), "disk detail");
    }
}
