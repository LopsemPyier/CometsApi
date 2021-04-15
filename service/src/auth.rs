use async_graphql::{FieldResult, Error};
use schema::dto::auth::{ LoginDto, RegisterDto };
use db::Database;
use utils::auth::{ create_token, verify_password };

pub async fn register(input: RegisterDto, db: &Database) -> FieldResult<String> {
    let user = db.get_user_by_email(&input.email.to_lowercase()).await?;

    if let None = user {
        let new = db.create_user(input.username, input.password, input.email.to_lowercase()).await?;
        return Ok(create_token(new.id, new.username));
    }

    Err(Error::new("Email already taken"))
}

pub async fn login(input: LoginDto, db: &Database) -> FieldResult<String> {
    let user = db.get_user_by_email(&input.email.to_lowercase()).await?;

    if let Some(user) = user {
        return if verify_password(&user.password, &input.password) {
            Ok(create_token(user.id, user.username))
        } else {
            Err(Error::new("Invalid password"))
        }
    }
    Err(Error::new("Invalid email"))
}