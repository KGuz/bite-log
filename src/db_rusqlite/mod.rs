pub mod database;
pub mod entry;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::{database::*, entry::*};
    use chrono::Local;
    use rusqlite::Result as SqlResult;

    #[test]
    fn rusqlite_test() -> SqlResult<()> {
        let db = Database::default();
        db.clear()?;

        let date = Local::now().date_naive();
        db.insert(&Bite::new(1, date, None, "test 1".to_string()))?;
        db.insert(&Bite::new(2, date, None, "test 2".to_string()))?;
        db.replace(&Bite::new(2, date, None, "test 2-1".to_string()))?;

        let entries = db.select::<Bite>()?;
        println!("{:?}", entries);
        Ok(())
    }
}
