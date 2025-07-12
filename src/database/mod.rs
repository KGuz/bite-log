pub mod abstraction;
pub mod models;
pub mod schema;

use abstraction::*;
use anyhow::Result;
use chrono::Local;
use diesel::prelude::*;
use models::*;

pub struct Database {
    connection: SqliteConnection,
    pub bites: Vec<Bite>,
    pub profiles: Vec<Profile>,
}

impl Database {
    pub fn new(connection: SqliteConnection) -> Self {
        Database {
            connection,
            bites: vec![],
            profiles: vec![],
        }
    }

    pub fn from(path: &str) -> Result<Self> {
        let mut connection = SqliteConnection::establish(path)?;
        const UP: &str = include_str!("sql/up.sql");

        query!(UP, connection);

        let mut db = Database::new(connection);
        // todo!("check databse schema");
        let today = Local::now().date_naive();
        db.bites = select!(bites where date < today, db.connection);
        db.profiles = read!(profiles, &mut db.connection);
        Ok(db)
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::{Local, NaiveDate};
    use diesel::{prelude::*, sql_query};

    const UP: &str = include_str!("sql/up.sql");
    const DOWN: &str = include_str!("sql/down.sql");

    fn chocolate() -> NewBite<'static> {
        let now = Local::now();
        NewBite {
            name: "Chocolate",
            calories: 207,
            category: Some(Category::Snack),
            nutritions: Some(Nutritions {
                fats: 14.,
                carbohydrates: 24.4,
                fibers: 2.3,
                proteins: 1.6,
            }),
            date: now.date_naive(),
            time: now.time(),
        }
    }

    #[test]
    fn playground() -> Result<()> {
        let mut conn = SqliteConnection::establish("assets/test.db").unwrap();
        query!(UP, conn);

        insert!(bites, chocolate(), &mut conn);
        insert!(bites, chocolate(), &mut conn);
        insert!(bites, chocolate(), &mut conn);
        update!(bites, 2, category = Category::Breakfast, &mut conn);
        delete!(bites, 1, &mut conn);

        println!("Displaying bites");
        for bite in read!(bites, &mut conn) {
            println!("{:?}", bite)
        }

        query!(DOWN, conn);
        Ok(())
    }
}
