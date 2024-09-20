use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::*;
use crate::schema::*;
pub struct UserRepository;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

impl UserRepository {
    pub async fn create(new_user: NewUser) -> QueryResult<User> {
        let mut c = load_db_connection().await;
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&mut c)
            .await?;

        Ok(user)
    }

    pub async fn list_users(c: &mut AsyncPgConnection) -> QueryResult<Vec<User>> {
        users::table.load(c).await
    }

    pub async fn find_by_username(username: &String) -> QueryResult<User> {
        let mut c = load_db_connection().await;
        users::table
            .filter(users::username.eq(username))
            .get_result(&mut c)
            .await
    }

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }
}
