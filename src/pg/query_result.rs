use super::{SpinPgTypeInfo, Connection};

use sqlx::ColumnIndex;

pub struct SpinPgRow {
    pub(crate) columns: std::sync::Arc<Vec<SpinPgColumn>>,
    pub(crate) inner: spin_sdk::pg::Row,
}

#[derive(Default)]
pub struct SpinPgQueryResult {
    pub(super) count: u64,
}

#[derive(Debug)]
pub struct SpinPgColumn {
    index: usize,
    column: spin_sdk::pg::Column,
}

impl SpinPgColumn {
    pub(crate) fn new(index: usize, column: &spin_sdk::pg::Column) -> Self {
        Self { index, column: column.clone() }
    }
}

impl sqlx::Row for SpinPgRow {
    type Database = Connection;

    fn columns(&self) -> &[<Self::Database as sqlx::Database>::Column] {
        &self.columns
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx::database::HasValueRef<'_>>::ValueRef, sqlx::Error>
    where
        I: sqlx::ColumnIndex<Self> {
        let uindex = index.index(&self)?;

        if uindex >= self.inner.len() {
            return Err(sqlx::Error::ColumnIndexOutOfBounds { index: uindex, len: self.inner.len() });
        }

        let val = &self.inner[uindex];
        Ok(SpinPgValue { inner: val.clone() })
    }
}

impl SpinPgQueryResult {
    pub fn count(&self) -> u64 {
        self.count
    }
}

impl Extend<SpinPgQueryResult> for SpinPgQueryResult {
    fn extend<T: IntoIterator<Item = SpinPgQueryResult>>(&mut self, iter: T) {
        for qr in iter.into_iter() {
            self.count += qr.count;
        }
    }
}

impl sqlx::Column for SpinPgColumn {
    type Database = Connection;

    fn ordinal(&self) -> usize {
        self.index
    }

    fn name(&self) -> &str {
        &self.column.name
    }

    fn type_info(&self) -> &<Self::Database as sqlx::Database>::TypeInfo {
        todo!()
    }
}

#[derive(Clone)]
pub struct SpinPgValue {
    pub(crate) inner: spin_sdk::pg::DbValue,
}

impl<'q> sqlx::ValueRef<'q> for SpinPgValue {
    type Database = Connection;

    fn to_owned(&self) -> <Self::Database as sqlx::Database>::Value {
        self.clone()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        let type_info = match &self.inner {
            spin_sdk::pg::DbValue::DbNull => SpinPgTypeInfo::Null,
            spin_sdk::pg::DbValue::Boolean(_) => SpinPgTypeInfo::Bool,
            spin_sdk::pg::DbValue::Int8(_) => SpinPgTypeInfo::Unsupported,
            spin_sdk::pg::DbValue::Int16(_) => SpinPgTypeInfo::Int16,
            spin_sdk::pg::DbValue::Int32(_) => SpinPgTypeInfo::Int32,
            spin_sdk::pg::DbValue::Int64(_) => SpinPgTypeInfo::Int64,
            spin_sdk::pg::DbValue::Uint8(_) => SpinPgTypeInfo::Unsupported,
            spin_sdk::pg::DbValue::Uint16(_) => SpinPgTypeInfo::Unsupported,
            spin_sdk::pg::DbValue::Uint32(_) => SpinPgTypeInfo::Unsupported,
            spin_sdk::pg::DbValue::Uint64(_) => SpinPgTypeInfo::Unsupported,
            spin_sdk::pg::DbValue::Floating32(_) => SpinPgTypeInfo::Floating32,
            spin_sdk::pg::DbValue::Floating64(_) => SpinPgTypeInfo::Floating64,
            spin_sdk::pg::DbValue::Str(_) => SpinPgTypeInfo::Str,
            spin_sdk::pg::DbValue::Binary(_) => SpinPgTypeInfo::Binary,
            spin_sdk::pg::DbValue::Unsupported => SpinPgTypeInfo::Null,
        };
        std::borrow::Cow::Owned(type_info)
    }

    fn is_null(&self) -> bool {
        matches!(&self.inner, spin_sdk::pg::DbValue::DbNull)
    }
}

impl sqlx::Value for SpinPgValue {
    type Database = Connection;

    fn as_ref(&self) -> <Self::Database as sqlx::database::HasValueRef<'_>>::ValueRef {
        todo!()
    }

    fn type_info(&self) -> std::borrow::Cow<'_, <Self::Database as sqlx::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl ColumnIndex<SpinPgRow> for usize {
    fn index(&self, container: &SpinPgRow) -> Result<usize, sqlx::Error> {
        if *self < container.inner.len() {
            Ok(*self)
        } else {
            Err(sqlx::Error::ColumnIndexOutOfBounds { index: *self, len: container.inner.len() })
        }
    }
}

impl ColumnIndex<SpinPgRow> for &str {
    fn index(&self, container: &SpinPgRow) -> Result<usize, sqlx::Error> {
        container.columns.iter().position(|c| &c.column.name == self)
            .ok_or_else(|| sqlx::Error::ColumnNotFound(self.to_string()))
    }
}
