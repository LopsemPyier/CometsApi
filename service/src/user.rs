use async_graphql::{ErrorExtensions, FieldResult};
use uuid::Uuid;

use db::Database;
use schema::error::common::CommonError;
use schema::object::user::UserObject;

pub async fn get_all(db: &Database) -> FieldResult<Vec<UserObject>> {
    let users_row = db.get_users().await?;

    Ok(users_row.into_iter().map(UserObject::from).collect())
}

pub async fn get_by_uuid(id: Uuid, db: &Database) -> FieldResult<UserObject> {
    let user_row = db.get_user_by_uuid(id).await?;

    if let Some(user) = user_row {
        Ok(UserObject::from(user))
    } else {
        Err(CommonError::NotFound(id).extend())
    }
}

pub async fn get_by_email(email: String, db: &Database) -> FieldResult<UserObject> {
    let user_row = db.get_user_by_email(&email).await?;

    if let Some(user) = user_row {
        Ok(UserObject::from(user))
    } else {
        Err(CommonError::NoEmail(email).extend())
    }
}

// pub async fn get_me(token: J)