use crate::auth::hash_password;
use crate::models::NewUser;
use crate::models::User;
use crate::repo::UserRepository;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role: String) -> QueryResult<User> {
    let password_hash = hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash,
        role,
    };

    let user = UserRepository::create(new_user).await.unwrap();

    Ok(user)
}

pub async fn list_all_users() -> QueryResult<Vec<User>> {
    let mut c = load_db_connection().await;

    let users = UserRepository::list_users(&mut c).await.unwrap();

    Ok(users)
}
