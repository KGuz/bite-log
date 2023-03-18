use crate::traits::{FromSql, ToSql};
use rusqlite::{types::ToSqlOutput, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl From<&'static str> for Database {
    fn from(path: &'static str) -> Self {
        let conn = Connection::open(path).expect("connection should not fail");
        let sql = "create table if not exists entries (
            id integer primary key,
            data text not null
        )";
        conn.execute(sql, []).unwrap();
        Database { conn }
    }
}

impl Default for Database {
    fn default() -> Self {
        Database::from("assets/bitelog.db")
    }
}

impl Database {
    pub fn select<T: FromSql>(&self) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare("select * from entries")?;
        let rows = stmt.query_map([], T::from_sql)?;
        Ok(rows.flatten().collect())
    }

    pub fn insert<T: ToSql>(&self, data: &T) -> Result<usize> {
        let (keys, params): (Vec<&str>, Vec<ToSqlOutput>) = data.to_sql().into_iter().unzip();
        let values = vec!["?"; params.len()];

        let keys = keys.join(",");
        let values = values.join(",");

        let sql = format!("insert into entries ({keys}) values ({values})");
        self.execute(&sql, params)
    }

    pub fn replace<T: ToSql>(&self, data: &T) -> Result<usize> {
        let (keys, params): (Vec<&str>, Vec<ToSqlOutput>) = data.to_sql().into_iter().unzip();
        let values = vec!["?"; params.len()];

        let keys = keys.join(",");
        let values = values.join(",");

        let sql = format!("replace into entries ({keys}) values ({values})");
        self.execute(&sql, params)
    }

    fn execute(&self, sql: &str, params: Vec<ToSqlOutput>) -> Result<usize> {
        macro_rules! match_params {
            ($($len: literal),*) => {
                match params.len() {
                    $(
                        $len => self.conn.execute(sql, TryInto::<[ToSqlOutput; $len]>::try_into(params).unwrap()),
                    )*
                    _ => std::unimplemented!(),
                }
            };
        }
        match_params!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
    }
}
