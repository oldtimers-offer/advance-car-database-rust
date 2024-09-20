#[macro_use]
extern crate rocket;
extern crate diesel;
use api::create_user_api;
use api::create_user_api2;
use api::get_users;
use api::login;
use diesel::prelude::*;
use diesel::Connection;
use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket_db_pools::Database;
use std::env;

mod api;
mod auth;
mod models;
mod repo;
mod schema;
mod users;

use models::{ClassicCar, NewClassicCar};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// GET all cars
#[get("/cars")]
async fn get_cars() -> Json<Vec<ClassicCar>> {
    use schema::classic_cars::dsl::*;

    let connection = &mut establish_connection();

    let cars = classic_cars
        .limit(100)
        .select(ClassicCar::as_select())
        .load(connection)
        .expect("Error loading cars");

    Json(cars)
}

// POST a new car
#[post("/cars", data = "<car>")]
async fn add_car(car: Json<ClassicCar>) -> Json<&'static str> {
    use schema::classic_cars;

    let connection = &mut establish_connection();

    let new_car = NewClassicCar {
        make: car.make.clone(),
        model: car.model.clone(),
        year: car.year,
        mileage: car.mileage,
    };

    diesel::insert_into(classic_cars::table)
        .values(&new_car)
        .execute(connection)
        .expect("Error inserting car");

    Json("Car added successfully")
}

// One way
// #[launch]
// fn rocket() -> _ {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     let pool = r2d2::Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.");

//     rocket::build()
//         .manage(pool)
//         .mount("/", routes![get_cars, add_car])
// }

// Another way
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                get_cars,
                add_car,
                create_user_api,
                create_user_api2,
                get_users,
                login
            ],
        )
        .attach(api::Db::init())
        .launch()
        .await?;

    Ok(())
}
