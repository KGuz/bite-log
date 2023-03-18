use crate::traits::{FromSql, ToSql};
use rusqlite::{
    types::{ToSql as ToSqlType, ToSqlOutput},
    Result, Row,
};

#[derive(Debug)]
pub struct TestEntry {
    pub id: u32,
    pub data: String,
}

impl TestEntry {
    pub fn new(id: u32, data: impl Into<String>) -> Self {
        TestEntry {
            id,
            data: data.into(),
        }
    }
}

impl FromSql for TestEntry {
    fn from_sql(row: &Row) -> Result<Self> {
        Ok(TestEntry {
            id: row.get(0)?,
            data: row.get(1)?,
        })
    }
}

impl ToSql for TestEntry {
    fn to_sql(&self) -> Vec<(&'static str, ToSqlOutput)> {
        vec![
            ("id", self.id.to_sql().unwrap()),
            ("data", self.data.to_sql().unwrap()),
        ]
    }
}
