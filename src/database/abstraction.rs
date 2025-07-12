macro_rules! query {
    ($sql: expr, $conn: expr) => {
        diesel::sql_query($sql).execute(&mut $conn)?
    };
}
pub(crate) use query;

macro_rules! read {
    (bites, $conn:expr) => {{
        use crate::database::{models::*, schema::bites::dsl};
        dsl::bites.load::<Bite>($conn)?
    }};
    (profiles, $conn:expr) => {{
        use crate::database::{models::*, schema::profiles::dsl};
        dsl::profiles.load::<Profile>($conn)?
    }};
}
pub(crate) use read;

macro_rules! select {
    ($table: tt where $column: tt == $value: expr, $conn:expr) => { select!($table where $column.eq($value), $conn) };
    ($table: tt where $column: tt != $value: expr, $conn:expr) => { select!($table where $column.ne($value), $conn) };
    ($table: tt where $column: tt <= $value: expr, $conn:expr) => { select!($table where $column.le($value), $conn) };
    ($table: tt where $column: tt <  $value: expr, $conn:expr) => { select!($table where $column.lt($value), $conn) };
    ($table: tt where $column: tt >= $value: expr, $conn:expr) => { select!($table where $column.ge($value), $conn) };
    ($table: tt where $column: tt >  $value: expr, $conn:expr) => { select!($table where $column.gt($value), $conn) };
    ($table: tt where $column: tt .$cmp: tt ($value: expr), $conn:expr) => {{
        use crate::database::schema::$table::dsl;
        dsl::$table
            .filter(dsl::$column.$cmp($value))
            .get_results(&mut $conn)?
    }};
}
pub(crate) use select;

macro_rules! insert {
    ($table: tt, $value: expr, $conn:expr) => {{
        use crate::database::schema::$table;
        diesel::insert_into($table::table)
            .values(&$value)
            .execute($conn)?
    }};
}
pub(crate) use insert;

macro_rules! update {
    ($table: tt, $id: expr, $column: tt = $value: expr, $conn:expr) => {{
        use crate::database::schema::$table::dsl;
        diesel::update(dsl::$table.filter(dsl::id.eq($id)))
            .set(dsl::$column.eq($value))
            .execute($conn)?;
    }};
}
pub(crate) use update;

macro_rules! delete {
    ($table: tt, $id: expr, $conn:expr) => {{
        use crate::database::schema::$table::dsl;
        diesel::delete(dsl::$table.filter(dsl::id.eq($id))).execute($conn)?
    }};
}
pub(crate) use delete;
