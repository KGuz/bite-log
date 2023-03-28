use super::schema::bites;
use chrono::{Local, NaiveDate, NaiveTime};
use diesel::prelude::*;

#[derive(Queryable, Debug, Clone)]
pub struct Bite {
    pub id: i32,
    pub name: String,
    pub calories: i32,
    pub category: Option<i32>,
    pub nutritions: Option<i32>,
    pub date: NaiveDate,
    pub time: NaiveTime,
}

#[derive(Insertable, Debug, Clone, Copy)]
#[diesel(table_name = bites)]
pub struct NewBite<'a> {
    pub name: &'a str,
    pub calories: i32,
    pub category: Option<i32>,
    pub nutritions: Option<i32>,
    pub date: NaiveDate,
    pub time: NaiveTime,
}


impl Default for NewBite<'_> {
    fn default() -> Self {
        let now = Local::now();
        NewBite {
            name: "new bite",
            calories: 0,
            category: None,
            nutritions: None,
            date: now.date_naive(),
            time: now.time()
        }
    }
}

// #[derive(Queryable, Debug, Clone)]
// pub struct Profile {
//     pub name: String,
//     pub height: u32,
//     pub weight: u32,
//     pub activity: ActivityLevel,
// }

// #[derive(Debug, Copy, Clone)]
// pub struct Nutritions {
//     pub fats: f32,
//     pub carbohydrates: f32,
//     pub fibers: f32,
//     pub proteins: f32,
// }

// #[derive(SqlType)]
// pub struct MyType;

// #[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression, Eq)]
// #[diesel(sql_type = MyType)]
// pub enum Category {
//     Breakfast,
//     Brunch,
//     Snack,
//     Lunch,
//     Supper,
//     Dinner,
// }

// #[derive(Debug, Copy, Clone)]
// pub enum ActivityLevel {
//     Sedentary,
//     LightlyActive,
//     ModeratelyActive,
//     VeryActive,
// }
