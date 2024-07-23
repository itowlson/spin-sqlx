use anyhow::anyhow;
use sqlx::error::DatabaseError;

pub(crate) trait AsSqlxResult<T> {
    fn as_sqlx_result(self) -> Result<T, sqlx::Error>;
}

impl<T, E: AsSqlxError> AsSqlxResult<T> for Result<T, E> {
    fn as_sqlx_result(self) -> Result<T, sqlx::Error> {
        self.map_err(|e| e.as_sqlx_error())
    }
}

trait AsSqlxError {
    fn as_sqlx_error(self) -> sqlx::Error;
}

impl AsSqlxError for spin_sdk::pg::Error {
    fn as_sqlx_error(self) -> sqlx::Error {
        match self {
            spin_sdk::pg::Error::PgError(err) => err.as_sqlx_error(),
            spin_sdk::pg::Error::Decode(msg) => sqlx::Error::Decode(anyhow!(msg).into()),
        }
    }
}

impl AsSqlxError for spin_sdk::pg::PgError {
    fn as_sqlx_error(self) -> sqlx::Error {
        match self {
            // TODO: better errors
            spin_sdk::pg::PgError::ConnectionFailed(msg) => io_error(&msg),
            spin_sdk::pg::PgError::QueryFailed(msg) => io_error(&msg),
            spin_sdk::pg::PgError::BadParameter(msg) => io_error(&msg),
            spin_sdk::pg::PgError::ValueConversionFailed(msg) => sqlx::Error::Decode(anyhow!(msg).into()),
            spin_sdk::pg::PgError::Other(msg) => io_error(&msg),
        }
    }
}

fn io_error(message: &str) -> sqlx::error::Error {
    sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, anyhow!(message.to_owned())))
}

struct DatabaseFull;

impl sqlx::error::DatabaseError for DatabaseFull {
    fn message(&self) -> &str {
        "Database full"
    }

    fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        todo!()
    }

    fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
        todo!()
    }

    fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        todo!()
    }

    fn kind(&self) -> sqlx::error::ErrorKind {
        sqlx::error::ErrorKind::Other
    }
}

impl std::error::Error for DatabaseFull {}

impl std::fmt::Debug for DatabaseFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::fmt::Display for DatabaseFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}
