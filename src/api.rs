use crate::models::NewUser;
use crate::repo::UserRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use serde_json::json;
use std::error::Error;

use crate::auth::authorize_user;
use crate::auth::Credentials;
use crate::users::create_user;
use crate::users::list_all_users;
use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("my_pg_db_name")]
pub struct Db(PgPool);

fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

// Create new User
#[rocket::post("/users", format = "json", data = "<new_user>")]
pub async fn create_user_api(new_user: Json<NewUser>) -> Result<Custom<Value>, Custom<Value>> {
    UserRepository::create(new_user.into_inner())
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}

// Create new User II way
#[rocket::post("/users2", data = "<new_user>")]
pub async fn create_user_api2(new_user: Json<NewUser>) -> Result<Custom<Value>, Custom<Value>> {
    create_user(
        new_user.username.clone(),
        new_user.password.clone(),
        new_user.role.clone(),
    )
    .await
    .map(|user| Custom(Status::Created, json!(user)))
    .map_err(|e| server_error(e.into()))
}

#[rocket::get("/all_users")]
pub async fn get_users() -> Result<Value, Custom<Value>> {
    list_all_users()
        .await
        .map(|users| json!(users))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Credentials>) -> Result<String, Custom<Value>> {
    let user = UserRepository::find_by_username(&credentials.username)
        .await
        .map_err(|_| Custom(Status::Unauthorized, json!("Missing user")))?;

    let session_id = authorize_user(&user, credentials)
        .await
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    Ok(session_id)
}
