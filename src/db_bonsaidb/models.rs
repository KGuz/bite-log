use bonsaidb::core::{
    document::{CollectionDocument, Emit},
    schema::{Collection, CollectionViewSchema, View, ViewMapResult},
};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

pub mod bites {

    use super::*;

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum Category {
        Breakfast,
        Brunch,
        Snack,
        Lunch,
        Supper,
        Dinner,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Nutritions {
        pub fats: f32,
        pub carbohydrates: f32,
        pub fibers: f32,
        pub proteins: f32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Collection)]
    #[collection(name = "bites")]
    pub struct Bite {
        pub name: String,
        pub calories: f32,
        pub category: Option<Category>,
        pub nutritions: Option<Nutritions>,
        pub date: NaiveDate,
        pub time: Option<NaiveTime>,
    }

    #[derive(Debug, Clone, View)]
    #[view(collection = Bite, key = String, value = NaiveDate, name = "by-date")]
    struct BitesByDate;

    impl CollectionViewSchema for BitesByDate {
        type View = Self;

        fn map(&self, document: CollectionDocument<Bite>) -> ViewMapResult<Self::View> {
            document
                .header
                .emit_key_and_value(document.contents.date.to_string(), document.contents.date)
        }
    }
}

pub mod profiles {
    use super::*;

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum ActivityLevel {
        Sedentary,
        LightlyActive,
        ModeratelyActive,
        VeryActive,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Collection)]
    #[collection(name = "profiles")]
    pub struct Profile {
        pub name: String,
        pub height: u32,
        pub weight: u32,
        pub activity: ActivityLevel,
    }
}
