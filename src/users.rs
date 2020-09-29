use super::schema::users::dsl::*;
use diesel::{prelude::*, SqliteConnection};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

type UserResult<T> = Result<T, diesel::result::Error>;

pub fn get_user_by_name(db: &SqliteConnection, user_name: String) -> UserResult<User> {
    users.filter(username.eq(user_name)).first(db)
}

pub fn get_users(db: &SqliteConnection) -> UserResult<Vec<User>> {
    users.load(db)
}
