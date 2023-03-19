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
        insert(&mut conn, "title", "description")?;
        insert(&mut conn, "title", "description")?;
        update(&mut conn, 2);
        delete(&mut conn, 1);
        read(&mut conn)?;

        sql_query(DOWN).execute(&mut conn)?;
        Ok(())
    }

    fn read(conn: &mut SqliteConnection) -> Result<()> {
        use schema::posts::dsl::*;

        let results = posts.load::<Post>(conn)?;

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{:?}", post);
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

    fn update(conn: &mut SqliteConnection, post_id: i32) -> Result<()> {
        use schema::posts::{self, dsl::*};

        diesel::update(posts.filter(id.eq(post_id)))
            .set(published.eq(true))
            .execute(conn)?;
        Ok(())
    }

    fn delete(conn: &mut SqliteConnection, post_id: i32) -> Result<()> {
        use schema::posts::{self, dsl::*};

        diesel::delete(posts.filter(id.eq(post_id))).execute(conn)?;
        Ok(())
    }
}
