pub mod model;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::{model::*, schema};
    use diesel::prelude::*;

    #[test]
    fn diesel_test() -> ConnectionResult<()> {
        use schema::posts::dsl::*;

        let mut connection = SqliteConnection::establish("assets/bitelog-diesel.db")?;
        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(&mut connection)
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
