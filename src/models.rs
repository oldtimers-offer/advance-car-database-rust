use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassicCar {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub mileage: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=classic_cars)]
pub struct NewClassicCar {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub mileage: i32,
}

// #[derive(Debug, Deserialize)]
// struct CreateCarRequest {
//     make: String,
//     model: String,
//     year: i32,
//     mileage: i32,
// }

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub role: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: String,
}
