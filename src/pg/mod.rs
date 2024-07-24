use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use log::LevelFilter;

mod convert;
mod error;
mod query_result;
mod type_info;

pub use query_result::{SpinPgColumn, SpinPgQueryResult, SpinPgRow, SpinPgValue};
pub use type_info::SpinPgTypeInfo;

use error::AsSqlxResult;

#[derive(Debug)]
pub struct Connection(spin_sdk::pg::Connection);

impl Connection {
    pub fn new(conn: spin_sdk::pg::Connection) -> Self {
        Self(conn)
    }

    pub fn open(address: &str) -> anyhow::Result<Self> {
        Ok(Self(spin_sdk::pg::Connection::open(address)?))
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionOptions {
    address: String,
}

impl sqlx::Connection for Connection {
    type Database = Connection;

    type Options = ConnectionOptions;

    fn close(self) -> BoxFuture<'static, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }
    fn close_hard(self) -> BoxFuture<'static, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<sqlx::Transaction<'_, Self::Database>, sqlx::Error>>
    where
        Self: Sized {
        todo!()
    }

    fn shrink_buffers(&mut self) {
    }
    fn flush(&mut self) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn should_flush(&self) -> bool { false }
}

impl sqlx::Database for Connection {
    type Connection = Connection;

    type TransactionManager = Connection;

    type Row = SpinPgRow;

    type QueryResult = SpinPgQueryResult;

    type Column = SpinPgColumn;

    type TypeInfo = SpinPgTypeInfo;

    type Value = SpinPgValue;

    type Arguments<'q> = SpinPgArgs;

    type ArgumentBuffer<'q> = Vec<spin_sdk::pg::ParameterValue>;

    type Statement<'q> = SpinPgStmt;

    type ValueRef<'r> = SpinPgValue;

    const NAME: &'static str = "Spin PostgreSQL";

    const URL_SCHEMES: &'static [&'static str] = &["spin-pg"];
}

#[derive(Default)]
pub struct SpinPgArgs {
    inner: Vec<spin_sdk::pg::ParameterValue>,
}

impl SpinPgArgs {
    fn as_slice(&self) -> &[spin_sdk::pg::ParameterValue] {
        &self.inner
    }
}

impl<'q> sqlx::Arguments<'q> for SpinPgArgs {
    type Database = Connection;

    fn reserve(&mut self, _additional: usize, _size: usize) {
    }

    fn add<T>(&mut self, value: T) -> Result<(), sqlx::error::BoxDynError>
    where
        T: 'q + sqlx::Encode<'q, Self::Database> + sqlx::Type<Self::Database>
    {
        let _ = value.encode_by_ref(&mut self.inner)?;
        Ok(())
    }
    
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'q> sqlx::IntoArguments<'q, Connection> for SpinPgArgs {
    fn into_arguments(self) -> <Connection as sqlx::Database>::Arguments<'q> {
        self
    }
}

#[derive(Clone, Default)]
pub struct SpinPgStmt {
    sql: String,
}

impl SpinPgStmt {
    fn new(sql: &str) -> Self {
        Self { sql: sql.to_owned() }
    }
}

impl<'q> sqlx::Statement<'q> for SpinPgStmt {
    type Database = Connection;

    fn to_owned(&self) -> <Self::Database as sqlx::Database>::Statement<'static> {
        self.clone()
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<either::Either<&[<Self::Database as sqlx::Database>::TypeInfo], usize>> {
        todo!("prepared statements are not implemented for Spin Postgres")
    }

    fn columns(&self) -> &[<Self::Database as sqlx::Database>::Column] {
        todo!("prepared statements are not implemented for Spin Postgres")
    }

    fn query(&self) -> sqlx::query::Query<'_, Self::Database, <Self::Database as sqlx::Database>::Arguments<'q>> {
        todo!("prepared statements are not implemented for Spin Postgres")
    }

    fn query_with<'s, A>(&'s self, _arguments: A) -> sqlx::query::Query<'s, Self::Database, A>
    where
        A: sqlx::IntoArguments<'s, Self::Database>
    {
        todo!("prepared statements are not implemented for Spin Postgres")
    }

    fn query_as<O>(
        &self,
    ) -> sqlx::query::QueryAs<'_, Self::Database, O, <Self::Database as sqlx::Database>::Arguments<'q>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>
    {
        todo!("prepared statements are not implemented for Spin SQLite")
    }

    fn query_as_with<'s, O, A>(&'s self, _arguments: A) -> sqlx::query::QueryAs<'s, Self::Database, O, A>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database>
    {
        todo!("prepared statements are not implemented for Spin SQLite")
    }

    fn query_scalar<O>(
        &self,
    ) -> sqlx::query::QueryScalar<'_, Self::Database, O, <Self::Database as sqlx::Database>::Arguments<'q>>
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>
    {
        todo!("prepared statements are not implemented for Spin SQLite")
    }

    fn query_scalar_with<'s, O, A>(&'s self, _arguments: A) -> sqlx::query::QueryScalar<'s, Self::Database, O, A>
    where
        (O,): for<'r> sqlx::FromRow<'r, <Self::Database as sqlx::Database>::Row>,
        A: sqlx::IntoArguments<'s, Self::Database>
    {
        todo!("prepared statements are not implemented for Spin SQLite")
    }
}

impl sqlx::TransactionManager for Connection {
    type Database = Connection;

    fn begin(
        _conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!("transactions are not implemented for Spin SQLite")
    }

    fn commit(
        _conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!("transactions are not implemented for Spin SQLite")
    }

    fn rollback(
        _conn: &mut <Self::Database as sqlx::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx::Error>> {
        todo!("transactions are not implemented for Spin SQLite")
    }

    fn start_rollback(_conn: &mut <Self::Database as sqlx::Database>::Connection) {
        todo!("transactions are not implemented for Spin SQLite")
    }
}

impl std::str::FromStr for ConnectionOptions {
    type Err = sqlx::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { address: s.to_owned() })
    }
}

impl sqlx::ConnectOptions for ConnectionOptions {
    type Connection = Connection;

    fn from_url(url: &url::Url) -> Result<Self, sqlx::Error> {
        let address = url.to_string();
        Ok(Self { address })
    }

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, sqlx::Error>>
    where
        Self::Connection: Sized
    {
        Box::pin(async move {
            spin_sdk::pg::Connection::open(&self.address)
                .map(|conn| Connection(conn))
                .map_err(|e| sqlx::Error::AnyDriverError(Box::new(e)))
        })
    }

    fn log_statements(self, _level: LevelFilter) -> Self {
        self
    }

    fn log_slow_statements(self, _level: LevelFilter, _duration: std::time::Duration) -> Self {
        self
    }
}

impl<'c> sqlx::Executor<'c> for &'c Connection {
    type Database = Connection;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>
    {
        tracing::debug!("FETCH-MANYing {}", query.sql());
        // The args-exec dance needs to go on the SqlxConnection object
        let args = match query.take_arguments() {
            Ok(a) => a.unwrap_or_default(),
            Err(e) => {
                return Box::pin(futures::stream::once(async move { Err(sqlx::Error::Encode(e)) }));
            }
        };
        let rs = match self.0.query(query.sql(), args.as_slice()).as_sqlx_result() {
            Ok(rs) => rs,
            Err(e) => {
                return Box::pin(futures::stream::once(async move { Err(e) }));
            }
        };

        // Okay this CANNOT return a QueryResult because fetch will filtermap any
        // Either::Lefts away because reasons.  We have to get the rows.

        let columns_core = rs.columns.iter().enumerate().map(|(i, c)| SpinPgColumn::new(i, c)).collect::<Vec<_>>();
        let columns = std::sync::Arc::new(columns_core);
        let rows = rs.rows.into_iter()
            .map(move |r| Ok(sqlx::Either::Right(SpinPgRow { columns: columns.clone(), inner: r })));
        Box::pin(futures::stream::iter(rows))
    }

    fn execute<'e, 'q: 'e, E: 'q>(
            self,
            mut query: E,
        ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::QueryResult, sqlx::Error>>
        where
            'c: 'e,
            E: sqlx::Execute<'q, Self::Database>,
    {
        tracing::debug!("EXECing {}", query.sql());
        let args = match query.take_arguments() {
            Ok(a) => a.unwrap_or_default(),
            Err(e) => {
                return Box::pin(async move { Err(sqlx::Error::Encode(e)) });
            }
        };
        let count = match self.0.execute(query.sql(), args.as_slice()).as_sqlx_result() {
            Ok(count) => count,
            Err(e) => {
                return Box::pin(async move { Err(e) });
            }
        };

        let qr = SpinPgQueryResult { count };
        let res = Ok(qr);
        Box::pin(async { res })
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>
    {
        tracing::debug!("FETCH-OPTIONALing {}", query.sql());
        let args = match query.take_arguments() {
            Ok(a) => a.unwrap_or_default(),
            Err(e) => {
                return Box::pin(async move { Err(sqlx::Error::Encode(e)) });
            }
        };
        let rs = match self.0.query(query.sql(), args.as_slice()).as_sqlx_result() {
            Ok(rs) => rs,
            Err(e) => {
                return Box::pin(async move { Err(e) });
            }
        };

        // TODO: deduplicate
        let columns_core = rs.columns.iter().enumerate().map(|(i, c)| SpinPgColumn::new(i, c)).collect::<Vec<_>>();
        let columns = std::sync::Arc::new(columns_core);
        let row = rs.rows.into_iter()
            .map(move |r| SpinPgRow { columns: columns.clone(), inner: r })
            .next();

        Box::pin(async { Ok(row) })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        _parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>>
    where
        'c: 'e {
        let stmt = SpinPgStmt::new(sql);
        Box::pin(async { Ok(stmt) })
    }

    fn describe<'e, 'q: 'e>(
        self,
        _sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e
    {
        todo!("Spin does not currently support sqlx database describe for offline macros")
    }
}
