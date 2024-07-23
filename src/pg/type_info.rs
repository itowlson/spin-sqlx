use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum SpinPgTypeInfo {
    Bool,
    Int16,
    Int32,
    Int64,
    Floating32,
    Floating64,
    Str,
    Binary,
    Null,
    Unsupported,
}

impl Display for SpinPgTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use sqlx::TypeInfo;
        f.write_str(self.name())
    }
}

impl sqlx::TypeInfo for SpinPgTypeInfo {
    fn is_null(&self) -> bool {
        *self == Self::Null
    }

    fn name(&self) -> &str {
        match self {
            Self::Bool => "boolean",
            Self::Int16 => "smallint",
            Self::Int32 => "int",
            Self::Int64 => "bigint",
            Self::Floating32 => "real",
            Self::Floating64 => "double precision",
            Self::Str => "text",
            Self::Binary => "bytea",
            Self::Null => "NULL",
            Self::Unsupported => "<unsupported>",
        }
    }
}
