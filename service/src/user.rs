use async_graphql::{FieldResult, ErrorExtensions};
use db::Database;
use schema::object::user::UserObject;
use uuid::Uuid;
use schema::error::common::CommonError;

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

// pub async fn get_me(token: J)