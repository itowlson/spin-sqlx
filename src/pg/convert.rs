use std::fmt::Display;

use super::{Connection, SpinPgTypeInfo};

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
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Str(self.to_string()));
        Ok(sqlx::encode::IsNull::No)
    }
}

impl sqlx::Type<Connection> for &str {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Str
    }
}

impl<'r> sqlx::Decode<'r, Connection> for String {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Str(s) => Ok(s),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl<'q> sqlx::Encode<'q, Connection> for String {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Str(self.to_string()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl sqlx::Type<Connection> for String {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Str
    }
}

// --- INTEGER TYPE CONVERSIONS ---
// TODO: these all follow the same pattern: could they be a macro?

// We cannot do unsigned integers as they cannot be encoded to the matching signed types (and encode() doesn't let us return an error for this)
// TODO: we could widen them?  Bit ugh though

impl<'q> sqlx::Encode<'q, Connection> for i16 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Int16((*self).into()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i16 {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Int16(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i16 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Int16
    }
}

impl<'q> sqlx::Encode<'q, Connection> for i32 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Int32((*self).into()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i32 {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Int16(n) => into_or_err(n),
            spin_sdk::pg::DbValue::Int32(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i32 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Int32
    }
}

impl<'q> sqlx::Encode<'q, Connection> for i64 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Int64((*self).into()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for i64 {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Int16(n) => into_or_err(n),
            spin_sdk::pg::DbValue::Int32(n) => into_or_err(n),
            spin_sdk::pg::DbValue::Int64(n) => into_or_err(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for i64 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Int64
    }
}

// --- END INTEGERS ---

impl<'q> sqlx::Encode<'q, Connection> for bool {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Boolean(*self));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for bool {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Boolean(b) => Ok(b),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for bool {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Bool
    }
}

impl<'q> sqlx::Encode<'q, Connection> for f32 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Floating32((*self).into()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for f32 {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Floating32(n) => Ok(n as f32),  // `as` is the best conversion we have
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for f32 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Floating32
    }
}

impl<'q> sqlx::Encode<'q, Connection> for f64 {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Floating64((*self).into()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'r> sqlx::Decode<'r, Connection> for f64 {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Floating64(n) => Ok(n),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for f64 {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Floating64
    }
}

impl<'q> sqlx::Encode<'q, Connection> for &[u8] {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Binary(self.to_vec()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl<'q, const N: usize> sqlx::Encode<'q, Connection> for &[u8; N] {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.push(spin_sdk::pg::ParameterValue::Binary(self.to_vec()));
        Ok(sqlx::encode::IsNull::No)
    }
}
impl sqlx::Type<Connection> for &[u8] {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Binary
    }
}
impl<const N: usize> sqlx::Type<Connection> for &[u8; N] {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Binary
    }
}
impl<'r> sqlx::Decode<'r, Connection> for Vec<u8> {
    fn decode(value: <Connection as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.inner {
            spin_sdk::pg::DbValue::Binary(v) => Ok(v),
            _ => Err(Box::new(BadTypeError)),
        }
    }
}
impl sqlx::Type<Connection> for Vec<u8> {
    fn type_info() -> <Connection as sqlx::Database>::TypeInfo {
        SpinPgTypeInfo::Binary
    }
}

impl<'q, T: sqlx::Encode<'q, Connection>> sqlx::Encode<'q, Connection> for Option<T> {
    fn encode_by_ref(&self, buf: &mut <Connection as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            Some(v) => v.encode_by_ref(buf),
            None => {
                buf.push(spin_sdk::pg::ParameterValue::DbNull);
                Ok(sqlx::encode::IsNull::Yes)
            }
        }
    }
}
