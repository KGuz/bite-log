use super::schema::{bites, profiles};
use chrono::{Local, NaiveDate, NaiveTime};
use diesel::sql_types::Binary;
use diesel::sqlite::{Sqlite, SqliteValue};
use diesel::{deserialize, prelude::*, serialize, sql_types::Integer, AsExpression};

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

impl Default for NewBite<'_> {
    fn default() -> Self {
        let now = Local::now();
        NewBite {
            name: "new bite",
            calories: 0,
            category: None,
            nutritions: Some(Nutritions {
                fats: 0.,
                carbohydrates: 0.,
                fibers: 0.,
                proteins: 0.,
            }),
            date: now.date_naive(),
            time: now.time(),
        }
    }
}

#[derive(Debug, Copy, Clone, AsExpression, PartialEq)]
#[diesel(sql_type = Binary)]
pub struct Nutritions {
    pub fats: f32,
    pub carbohydrates: f32,
    pub fibers: f32,
    pub proteins: f32,
}

impl TryFrom<Vec<f32>> for Nutritions {
    type Error = &'static str;
    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err("Invalid vector length to perform conversion");
        }

        Ok(Nutritions {
            fats: value[0],
            carbohydrates: value[1],
            fibers: value[2],
            proteins: value[3],
        })
    }
}

impl serialize::ToSql<Binary, Sqlite> for Nutritions {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        let data = [
            self.fats.to_be_bytes(),
            self.carbohydrates.to_be_bytes(),
            self.fibers.to_be_bytes(),
            self.proteins.to_be_bytes(),
        ];

        out.set_value(data.concat());
        Ok(serialize::IsNull::No)
    }
}

impl deserialize::FromSql<Binary, Sqlite> for Nutritions {
    fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
        let data: Vec<f32> = <Vec<u8>>::from_sql(bytes)?
            .chunks_exact(4)
            .map(|chunk| f32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Nutritions::try_from(data).map_err(|err| err.into())
    }
}

#[derive(Debug, Clone, Copy, AsExpression, PartialEq, Eq)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum Category {
    Breakfast = 1,
    Brunch = 2,
    Snack = 3,
    Lunch = 4,
    Supper = 5,
    Dinner = 6,
}

impl TryFrom<i32> for Category {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Category::Breakfast),
            2 => Ok(Category::Brunch),
            3 => Ok(Category::Snack),
            4 => Ok(Category::Lunch),
            5 => Ok(Category::Supper),
            6 => Ok(Category::Dinner),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl serialize::ToSql<Integer, Sqlite> for Category {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self as i32);
        Ok(serialize::IsNull::No)
    }
}

impl deserialize::FromSql<Integer, Sqlite> for Category {
    fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        Category::try_from(value).map_err(|err| err.into())
    }
}

#[derive(Queryable, Selectable, Debug, Clone)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub height: i32,
    pub weight: i32,
    pub activity: ActivityLevel,
}

#[derive(Insertable, Debug, Clone)]
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
    Sedentary = 1,
    LightlyActive = 2,
    ModeratelyActive = 3,
    VeryActive = 4,
}

impl TryFrom<i32> for ActivityLevel {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ActivityLevel::Sedentary),
            2 => Ok(ActivityLevel::LightlyActive),
            3 => Ok(ActivityLevel::ModeratelyActive),
            4 => Ok(ActivityLevel::VeryActive),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl serialize::ToSql<Integer, Sqlite> for ActivityLevel {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self as i32);
        Ok(serialize::IsNull::No)
    }
}

impl deserialize::FromSql<Integer, Sqlite> for ActivityLevel {
    fn from_sql(bytes: SqliteValue) -> deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        ActivityLevel::try_from(value).map_err(|err| err.into())
    }
}
