use crate::models::User;
use argon2::password_hash::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub async fn authorize_user(user: &User, credentials: Json<Credentials>) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

    Ok("Success login".to_string())
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let b_password = password.as_bytes();
    let hashed_password = argon2.hash_password(b_password, &salt)?.to_string();
    Ok(hashed_password)
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for User {
//     type Error = ();

//     async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         //Authorization: Bearer SESSION ID 128 CHARACTERS LONG
//         let session_header = req
//             .headers()
//             .get_one("Authorization")
//             .map(|v| v.split_whitespace().collect::<Vec<_>>())
//             .filter(|v| v.len() == 2 && v[0] == "Bearer");
//         if let Some(header_value) = session_header {
//             let mut cache = req
//                 .guard::<Connection<CacheConn>>()
//                 .await
//                 .expect("Can not connect to Redis in request guard");
//             let mut db = req
//                 .guard::<Connection<Db>>()
//                 .await
//                 .expect("Can not connect to Postgres in request guard");
//             //.map_error(|_| Custom(Status::Unauthorized, json!("Can not connect to Postgres in request guard")));

//             let result = cache
//                 .get::<String, i32>(format!("sessions/{}", header_value[1]))
//                 .await;

//             if let Ok(user_id) = result {
//                 if let Ok(user) = UserRepository::find(&mut db, user_id).await {
//                     return Outcome::Success(user);
//                 }
//             }
//         }

//         Outcome::Error((Status::Unauthorized, ()))
//     }
// }
