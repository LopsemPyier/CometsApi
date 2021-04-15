use async_graphql::*;

#[derive(InputObject)]
/// The DTO for registration
pub struct RegisterDto {
    /// Username
    pub username: String,
    /// Email (must not already be in the database)
    pub email: String,
    /// Password
    pub password: String
}

#[derive(InputObject)]
/// The DTO for login
pub struct LoginDto {
    /// Email
    pub email: String,
    /// Password
    pub password: String
}