pub mod models;
pub mod schema;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::{Local, NaiveDate};
    use diesel::{prelude::*, sql_query};

    #[test]
    fn playground() -> Result<()> {
        let mut conn = SqliteConnection::establish("assets/test.db").unwrap();
        let setup = include_str!("sql/up.sql");
        sql_query(setup).execute(&mut conn)?;

        show(&mut conn);

        let drop = include_str!("sql/down.sql");
        sql_query(drop).execute(&mut conn)?;
        Ok(())
    }

    fn show(conn: &mut SqliteConnection) -> Result<()> {
        use {models::*, schema::posts::dsl::*};

        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(conn)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
        Ok(())
    }
}
