use super::schema::{bites, profiles};
use chrono::{NaiveDate, NaiveTime};
use core::{mem, ptr, slice};
use diesel::{
    deserialize::{self, FromSql},
    expression::AsExpression,
    prelude::*,
    serialize::{self, ToSql},
    sql_types::{Binary, Integer},
    sqlite::{Sqlite, SqliteValue},
};

pub mod bite {
    use super::*;

    #[derive(Queryable, Selectable, Debug, Clone)]
    pub struct Bite {
        pub id: i32,
        pub name: String,
        pub calories: i32,
        pub category: Option<Category>,
        pub nutritions: Option<Nutritions>,
        pub date: NaiveDate,
        pub time: NaiveTime,
    }

    #[derive(Insertable, Debug, Clone)]
    #[diesel(table_name = bites)]
    pub struct NewBite<'a> {
        pub name: &'a str,
        pub calories: i32,
        pub category: Option<Category>,
        pub nutritions: Option<Nutritions>,
        pub date: NaiveDate,
        pub time: NaiveTime,
    }

    #[derive(Debug, Clone, Copy, AsExpression, PartialEq, Eq)]
    #[diesel(sql_type = Integer)]
    #[repr(i32)]
    pub enum Category {
        Breakfast,
        Snack,
        Lunch,
        Supper,
        Dinner,
    }

    impl ToSql<Integer, Sqlite> for Category {
        fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
            out.set_value(*self as i32);
            Ok(serialize::IsNull::No)
        }
    }

    impl FromSql<Integer, Sqlite> for Category {
        fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
            let value = i32::from_sql(bytes)?;
            Ok(unsafe { mem::transmute(value) })
        }
    }

    #[derive(Debug, Copy, Clone, AsExpression, PartialEq)]
    #[diesel(sql_type = Binary)]
    #[repr(packed)]
    pub struct Nutritions {
        pub fats: f32,
        pub carbohydrates: f32,
        pub fibers: f32,
        pub proteins: f32,
    }

    impl ToSql<Binary, Sqlite> for Nutritions {
        fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
            let data = (self as *const _) as *const u8;
            let len = mem::size_of::<Self>();

            out.set_value(unsafe { slice::from_raw_parts(data, len) });
            Ok(serialize::IsNull::No)
        }
    }

    impl FromSql<Binary, Sqlite> for Nutritions {
        fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
            let value = <*const [u8]>::from_sql(bytes)?;
            Ok(unsafe { ptr::read(value as *const _) })
        }
    }
}
pub use bite::*;

pub mod profile {
    use super::*;
    #[derive(Queryable, Selectable, Debug, Clone, PartialEq, Eq)]
    pub struct Profile {
        pub id: i32,
        pub name: String,
        pub height: i32,
        pub weight: i32,
        pub activity: ActivityLevel,
    }

    #[derive(Insertable, Debug, Clone, Copy, PartialEq, Eq)]
    #[diesel(table_name = profiles)]
    pub struct NewProfile<'a> {
        pub name: &'a str,
        pub height: i32,
        pub weight: i32,
        pub activity: ActivityLevel,
    }

    #[derive(Debug, Clone, Copy, AsExpression, PartialEq, Eq)]
    #[diesel(sql_type = Integer)]
    #[repr(i32)]
    pub enum ActivityLevel {
        Sedentary,
        LightlyActive,
        ModeratelyActive,
        VeryActive,
    }

    impl ToSql<Integer, Sqlite> for ActivityLevel {
        fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
            out.set_value(*self as i32);
            Ok(serialize::IsNull::No)
        }
    }

    impl FromSql<Integer, Sqlite> for ActivityLevel {
        fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
            let value = i32::from_sql(bytes)?;
            Ok(unsafe { mem::transmute(value) })
        }
    }
}
pub use profile::*;
