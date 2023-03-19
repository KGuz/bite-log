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

        insert(&mut conn, "title", "description")?;
        show(&mut conn)?;

        sql_query(DOWN).execute(&mut conn)?;
        Ok(())
    }

    fn show(conn: &mut SqliteConnection) -> Result<()> {
        use schema::posts::dsl::*;

        let results = posts
            .filter(published.eq(false))
            .limit(5)
            .load::<Post>(conn)?;

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------");
            println!("{}", post.body);
        }
        Ok(())
    }

    fn insert(conn: &mut SqliteConnection, title: &str, body: &str) -> Result<()> {
        use schema::posts;

        let new_post = NewPost { title, body };
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;
        Ok(())
    }
}
