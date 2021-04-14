use async_graphql::{FieldResult, Error};
use db::Database;
use model::user::User;
use uuid::Uuid;

pub async fn get_all(db: &Database) -> FieldResult<Vec<User>> {
    let users_row = db.get_users().await?;

    Ok(users_row)
}

pub async fn get_by_uuid(id: Uuid, db: &Database) -> FieldResult<User> {
    let user_row = db.get_user_by_uuid(id).await?;

    if let Some(user) = user_row {
        Ok(user)
    } else {
        Err(Error::new("User not found"))
    }
}