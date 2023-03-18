use bite_log::{database::*, entry::*};
use rusqlite::Result;

fn main() -> Result<()> {
    let db = Database::default();
    db.clear()?;

    db.insert(&TestEntry::new(1, "test 1"))?;
    db.insert(&TestEntry::new(2, "test 2"))?;
    db.replace(&TestEntry::new(2, "test 2-1"))?;

    let entries = db.select::<TestEntry>()?;
    println!("{:?}", entries);
    Ok(())
}
