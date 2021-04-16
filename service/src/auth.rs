use async_graphql::{FieldResult, ErrorExtensions};
use schema::dto::auth::{ LoginDto, RegisterDto };
use db::Database;
use utils::auth::{ create_token, verify_password };
use schema::error::auth::AuthError;

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
        }
    }
    Err(AuthError::InvalidUsername.extend())
}