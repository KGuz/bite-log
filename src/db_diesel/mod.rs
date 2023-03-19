pub mod model;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::{model::*, schema};
    use diesel::{prelude::*, sql_query};

    #[test]
    fn diesel_test() -> ConnectionResult<()> {
        use schema::posts::dsl::*;
        let mut conn = SqliteConnection::establish("assets/bitelog-diesel.db")?;

        let create_table = include_str!("up.sql");
        sql_query(create_table).execute(&mut conn).unwrap();

        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(&mut conn)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }
        Ok(())
    }
}
