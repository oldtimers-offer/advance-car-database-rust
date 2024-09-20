// @generated automatically by Diesel CLI.

diesel::table! {
    classic_cars (id) {
        id -> Int4,
        make -> Varchar,
        model -> Varchar,
        year -> Int4,
        mileage -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        role -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classic_cars,
    users,
);
