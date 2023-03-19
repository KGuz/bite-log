use super::traits::{FromSql, ToSql};
use rusqlite::{types::ToSqlOutput, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl From<&'static str> for Database {
    fn from(path: &'static str) -> Self {
        let conn = Connection::open(path).expect("connection should not fail");
        let sql = "create table if not exists entries (
            id integer primary key,
            date text,
            time text null,
            snack text
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

    // Connection::execute doesnt accept dynamic sized params
    #[rustfmt::skip]
    fn execute(&self, sql: &str, params: Vec<ToSqlOutput>) -> Result<usize> {
        match params.len() {
            0 => self.conn.execute(sql, []),
            1 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 1]>::try_into(params).unwrap()),
            2 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 2]>::try_into(params).unwrap()),
            3 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 3]>::try_into(params).unwrap()),
            4 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 4]>::try_into(params).unwrap()),
            5 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 5]>::try_into(params).unwrap()),
            6 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 6]>::try_into(params).unwrap()),
            7 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 7]>::try_into(params).unwrap()),
            8 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 8]>::try_into(params).unwrap()),
            9 => self.conn.execute(sql, TryInto::<[ToSqlOutput; 9]>::try_into(params).unwrap()),
            _ => panic!("Too many parameters"),
        }
    }
}
