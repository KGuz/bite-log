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
    pub fn clear(&self) -> Result<usize> {
        self.conn.execute("delete from entries", [])
    }

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
}

// Connection::execute doesnt accept dynamic sized params
macro_rules! impl_execute {
    ($($len: literal),*) => {
        impl Database {
            fn execute(&self, sql: &str, params: Vec<ToSqlOutput>) -> Result<usize> {
                match params.len() {
                    0 => self.conn.execute(sql, []),
                    $(
                        $len => self.conn.execute(sql, TryInto::<[ToSqlOutput; $len]>::try_into(params).unwrap()),
                    )*
                    _ => unimplemented!(),
                }
            }
        }
    };
}
impl_execute!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);
