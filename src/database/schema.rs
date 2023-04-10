diesel::table! {
    bites (id) {
        id -> Integer,
        name -> Text,
        calories -> Integer,
        category -> Nullable<Integer>,
        nutritions -> Nullable<Binary>,
        date -> Date,
        time -> Time,

    }
}

diesel::table! {
    profiles (id) {
        id -> Integer,
        name -> Text,
        height -> Integer,
        weight -> Integer,
        activity -> Integer,
    }
}
