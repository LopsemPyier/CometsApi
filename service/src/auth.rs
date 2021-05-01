use async_graphql::{ErrorExtensions, FieldResult};
use uuid::Uuid;

use db::Database;
use schema::dto::auth::{LoginDto, RegisterDto};
use schema::error::auth::AuthError;
use utils::auth::{create_token, verify_password};

pub async fn register(input: RegisterDto, db: &Database) -> FieldResult<String> {
    let user = db.get_user_by_email(&input.email.to_lowercase()).await?;

    if let None = user {
        let new = db.create_user(input.username, input.password, input.email.to_lowercase()).await?;
        return Ok(create_token(new.id, new.username));
    }

    Err(AuthError::EmailTaken.extend())
}

pub async fn login(input: LoginDto, db: &Database) -> FieldResult<String> {
    let user = db.get_user_by_email(&input.email.to_lowercase()).await?;

    if let Some(user) = user {
        return if verify_password(&user.password, &input.password) {
            Ok(create_token(user.id, user.username))
        } else {
            Err(AuthError::InvalidPassword.extend())
        };
    }
    Err(AuthError::InvalidUsername.extend())
}

pub async fn update_password(db: &Database, id: Uuid, old: String, new: String) -> FieldResult<bool> {
    let user = db.get_user_by_uuid(id).await?;

    if let Some(user) = user {
        return if verify_password(&user.password, &old) {
            Ok(db.update_password(id, &new).await?)
        } else {
            Err(AuthError::InvalidPassword.extend())
        };
    }
    Err(AuthError::InvalidUsername.extend())
}

pub async fn update(db: &Database, id: Uuid, username: String, email: String) -> FieldResult<String> {
    let user = db.get_user_by_uuid(id).await?;

    if let Some(_user) = user {
        db.update_user(id, &username, &email).await?;
        return Ok(create_token(id, username));
    }
    Err(AuthError::InvalidUsername.extend())
}