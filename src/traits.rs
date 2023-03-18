use rusqlite::{types::ToSqlOutput, Result, Row};

pub trait FromSql {
    fn from_sql(row: &Row) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToSql {
    fn to_sql(&self) -> Vec<(&'static str, ToSqlOutput)>;
}
