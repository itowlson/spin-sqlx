use super::{SpinSqliteTypeInfo, Connection};

use sqlx::ColumnIndex;

pub struct SpinSqliteRow {
    pub(crate) columns: std::sync::Arc<Vec<SpinSqliteColumn>>,
    pub(crate) inner: spin_sdk::sqlite::RowResult,
}

#[derive(Default)]
pub struct SpinSqliteQueryResult {
    pub(crate) inner: Option<spin_sdk::sqlite::QueryResult>,  // Option because we can't construct a default one
}

#[derive(Debug, PartialEq)]
pub struct SpinSqliteColumn {
    index: usize,
    name: String,
}

impl SpinSqliteColumn {
    pub(crate) fn new(index: usize, name: &str) -> Self {
        Self { index, name: name.to_owned() }
    }
}

impl sqlx::Row for SpinSqliteRow {
    type Database = Connection;

    fn columns(&self) -> &[<Self::Database as sqlx::Database>::Column] {
        &self.columns
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx::Database>::ValueRef<'_>, sqlx::Error>
    where
        I: sqlx::ColumnIndex<Self> {
        let uindex = index.index(&self)?;

        if uindex >= self.inner.values.len() {
            return Err(sqlx::Error::ColumnIndexOutOfBounds { index: uindex, len: self.inner.values.len() });
        }

        let val = &self.inner.values[uindex];
        Ok(SpinSqliteValue { inner: val.clone() })
    }
}

impl Extend<SpinSqliteQueryResult> for SpinSqliteQueryResult {
    fn extend<T: IntoIterator<Item = SpinSqliteQueryResult>>(&mut self, iter: T) {
        for mut qr in iter.into_iter() {
            match self.inner.as_mut() {
                None => self.inner = qr.inner,
                Some(existing) => if let Some(qr) = qr.inner.as_mut() {
                    existing.rows.append(&mut qr.rows)
                },
            }
        }
    }
}

impl sqlx::Column for SpinSqliteColumn {
    type Database = Connection;

    fn ordinal(&self) -> usize {
        self.index
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &<Self::Database as sqlx::Database>::TypeInfo {
        todo!()
    }
}

#[derive(Clone)]
pub struct SpinSqliteValue {
    pub(crate) inner: spin_sdk::sqlite::Value,
}

impl<'q> sqlx::ValueRef<'q> for SpinSqliteValue {
    type Database = Connection;

    fn to_owned(&self) -> <Self::Database as sqlx::Database>::Value {
        self.clone()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        let type_info = match &self.inner {
            spin_sdk::sqlite::Value::Null => SpinSqliteTypeInfo::Null,
            spin_sdk::sqlite::Value::Integer(_) => SpinSqliteTypeInfo::Int,
            spin_sdk::sqlite::Value::Blob(_) => SpinSqliteTypeInfo::Blob,
            spin_sdk::sqlite::Value::Real(_) => SpinSqliteTypeInfo::Real,
            spin_sdk::sqlite::Value::Text(_) => SpinSqliteTypeInfo::Text,
        };
        std::borrow::Cow::Owned(type_info)
    }

    fn is_null(&self) -> bool {
        matches!(&self.inner, spin_sdk::sqlite::Value::Null)
    }
}

impl sqlx::Value for SpinSqliteValue {
    type Database = Connection;

    fn as_ref(&self) -> <Self::Database as sqlx::Database>::ValueRef<'_> {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl ColumnIndex<SpinSqliteRow> for usize {
    fn index(&self, container: &SpinSqliteRow) -> Result<usize, sqlx::Error> {
        if *self < container.inner.values.len() {
            Ok(*self)
        } else {
            Err(sqlx::Error::ColumnIndexOutOfBounds { index: *self, len: container.inner.values.len() })
        }
    }
}

impl ColumnIndex<SpinSqliteRow> for &str {
    fn index(&self, container: &SpinSqliteRow) -> Result<usize, sqlx::Error> {
        container.columns.iter().position(|c| &c.name == self)
            .ok_or_else(|| sqlx::Error::ColumnNotFound(self.to_string()))
    }
}
