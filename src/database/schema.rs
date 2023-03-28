diesel::table! {
    bites (id) {
        id -> Integer,
        name -> Text,
        calories -> Integer,
        category -> Nullable<Integer>,
        nutritions -> Nullable<Integer>,
        date -> Date,
        time -> Time,
    }
}
