use std::fmt::Display;

use super::{Connection, SpinSqliteTypeInfo};

// anyhow::Error makes sqlx mad
#[derive(Debug)]
struct BadTypeError;
impl std::error::Error for BadTypeError {}
impl Display for BadTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bad type")
    }
}

#[derive(Debug)]
struct BadValError;
impl std::error::Error for BadValError {}
impl Display for BadValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bad value")
    }
}

fn into_or_err<T: TryInto<U>, U>(value: T) -> Result<U, sqlx::error::BoxDynError> {
    match value.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(Box::new(BadValError)),  // TODO: how to get info out of e?
    }
}

impl<'q> sqlx::Encode<'q, Connection> for &str {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Text(self.to_string()));
        sqlx::encode::IsNull::No
    }
}

impl sqlx::Type<Connection> for &str {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Text
    }
}

impl<'r> sqlx::Decode<'r, Connection> for String {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Text(s) => Ok(s),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl<'q> sqlx::Encode<'q, Connection> for String {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Text(self.to_string()));
        sqlx::encode::IsNull::No
    }
}
impl sqlx::Type<Connection> for String {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Text
    }
}

// --- INTEGER TYPE CONVERSIONS ---
// TODO: these all follow the same pattern: could they be a macro?

impl<'q> sqlx::Encode<'q, Connection> for i16 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i16 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i16 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, Connection> for u16 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for u16 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for u16 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, Connection> for i32 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i32 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i32 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, Connection> for u32 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for u32 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for u32 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, Connection> for i64 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i64 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i64 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

// We cannot do u64 as it cannot be encoded to an i64 (and encode() doesn't let us return an error for this)

// --- END INTEGERS ---

impl<'q> sqlx::Encode<'q, Connection> for bool {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Integer(if *self { 1 } else { 0 }));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for bool {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Integer(0) => Ok(false),
            spin_sdk::sqlite::Value::Integer(1) => Ok(true),
            spin_sdk::sqlite::Value::Integer(_) => Err(Box::new(BadValError)),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for bool {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Int
    }
}

impl<'q> sqlx::Encode<'q, Connection> for f32 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Real((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for f32 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Real(n) => Ok(n as f32),  // `as` is the best conversion we have
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for f32 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Real
    }
}

impl<'q> sqlx::Encode<'q, Connection> for f64 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Real((*self).into()));
        sqlx::encode::IsNull::No
    }
}
impl<'r> sqlx::Decode<'r, Connection> for f64 {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Real(n) => Ok(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for f64 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Real
    }
}

impl<'q> sqlx::Encode<'q, Connection> for &[u8] {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Blob(self.to_vec()));
        sqlx::encode::IsNull::No
    }
}
impl<'q, const N: usize> sqlx::Encode<'q, Connection> for &[u8; N] {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        buf.push(spin_sdk::sqlite::Value::Blob(self.to_vec()));
        sqlx::encode::IsNull::No
    }
}
impl sqlx::Type<Connection> for &[u8] {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}
impl<const N: usize> sqlx::Type<Connection> for &[u8; N] {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}
impl<'r> sqlx::Decode<'r, Connection> for Vec<u8> {
    fn decode(value: <Connection as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::sqlite::Value::Blob(v) => Ok(v),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for Vec<u8> {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinSqliteTypeInfo::Blob
    }
}

impl<'q, T: sqlx::Encode<'q, Connection>> sqlx::Encode<'q, Connection> for Option<T> {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        match self {
            Some(v) => v.encode_by_ref(buf),
            None => {
                buf.push(spin_sdk::sqlite::Value::Null);
                sqlx::encode::IsNull::Yes
            }
        }
    }
}
