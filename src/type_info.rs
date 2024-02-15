use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum SpinSqliteTypeInfo {
    Int,
    Blob,
    Real,
    Text,
    Null,
}

impl Display for SpinSqliteTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use sqlx::TypeInfo;
        f.write_str(self.name())
    }
}

impl sqlx::TypeInfo for SpinSqliteTypeInfo {
    fn is_null(&self) -> bool {
        *self == Self::Null
    }

    fn name(&self) -> &str {
        match self {
            Self::Blob => "BINARY",
            Self::Int => "INT",
            Self::Null => "NULL",
            Self::Real => "REAL",
            Self::Text => "TEXT",
        }
    }
}
