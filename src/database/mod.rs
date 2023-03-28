pub mod models;
pub mod schema;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::{Local, NaiveDate};
    use diesel::{prelude::*, sql_query};
    use models::*;

    const UP: &str = include_str!("sql/up.sql");
    const DOWN: &str = include_str!("sql/down.sql");

    #[test]
    fn playground() -> Result<()> {
        let mut conn = SqliteConnection::establish("assets/test.db").unwrap();
        sql_query(UP).execute(&mut conn)?;

        insert(&mut conn, NewBite::default())?;
        insert(&mut conn, NewBite::default())?;
        insert(&mut conn, NewBite::default())?;
        update(&mut conn, 2);
        delete(&mut conn, 1);
        read(&mut conn)?;

        sql_query(DOWN).execute(&mut conn)?;
        Ok(())
    }

    fn read(conn: &mut SqliteConnection) -> Result<()> {
        use schema::bites::dsl::*;

        let results = bites.load::<Bite>(conn)?;

        println!("Displaying {} bites", results.len());
        for post in results {
            println!("{:?}", post);
        }
        Ok(())
    }

    fn insert(conn: &mut SqliteConnection, new_bite: NewBite) -> Result<()> {
        use schema::bites;

        diesel::insert_into(bites::table)
            .values(&new_bite)
            .execute(conn)?;
        Ok(())
    }

    fn update(conn: &mut SqliteConnection, bite_id: i32) -> Result<()> {
        use schema::bites::{self, dsl::*};

        diesel::update(bites.filter(id.eq(bite_id)))
            .set(calories.eq(1))
            .execute(conn)?;
        Ok(())
    }

    fn delete(conn: &mut SqliteConnection, bite_id: i32) -> Result<()> {
        use schema::bites::{self, dsl::*};

        diesel::delete(bites.filter(id.eq(bite_id))).execute(conn)?;
        Ok(())
    }
}
