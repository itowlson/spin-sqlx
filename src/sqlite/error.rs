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

impl AsSqlxError for spin_sdk::sqlite::Error {
    fn as_sqlx_error(self) -> sqlx::Error {
        match self {
            spin_sdk::sqlite::Error::AccessDenied => io_error("Component does not have access to database"),
            spin_sdk::sqlite::Error::DatabaseFull => sqlx::Error::Database(Box::new(DatabaseFull)),
            spin_sdk::sqlite::Error::InvalidConnection => io_error("Invalid connection handle"),
            spin_sdk::sqlite::Error::Io(e) => sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)),
            spin_sdk::sqlite::Error::NoSuchDatabase => io_error("No such database"),
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

pub(crate) trait UrlParseResult<T> {
    fn as_sqlx_result(self) -> Result<T, sqlx::Error>;
}

impl<T> UrlParseResult<url::Host<T>> for Option<url::Host<T>> {
    fn as_sqlx_result(self) -> Result<url::Host<T>, sqlx::Error> {
        match self {
            None => Err(sqlx::Error::Configuration(anyhow!("Invalid URL").into())),
            Some(val) => Ok(val)
        }
    }
}
