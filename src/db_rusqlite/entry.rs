use super::traits::{FromSql, ToSql};
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{
    types::{ToSql as ToSqlType, ToSqlOutput},
    Result, Row,
};

#[derive(Debug)]
pub struct Bite {
    pub id: u32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub snack: String,
}

impl Bite {
    pub fn new(id: u32, date: NaiveDate, time: Option<NaiveTime>, snack: String) -> Self {
        Bite {
            id,
            date,
            time,
            snack,
        }
    }
}

impl FromSql for Bite {
    fn from_sql(row: &Row) -> Result<Self> {
        Ok(Bite {
            id: row.get(0)?,
            date: row.get(1)?,
            time: row.get(2)?,
            snack: row.get(3)?,
        })
    }
}

impl ToSql for Bite {
    fn to_sql(&self) -> Vec<(&'static str, ToSqlOutput)> {
        vec![
            ("id", self.id.to_sql().unwrap()),
            ("date", self.date.to_sql().unwrap()),
            ("time", self.time.to_sql().unwrap()),
            ("snack", self.snack.to_sql().unwrap()),
        ]
    }
}
