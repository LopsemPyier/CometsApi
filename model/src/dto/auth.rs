use async_graphql::*;

#[derive(InputObject)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(InputObject)]
pub struct LoginDto {
    pub email: String,
    pub password: String
}