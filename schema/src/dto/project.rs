use async_graphql::*;
use uuid::Uuid;

#[derive(InputObject)]
/// The DTO for project
pub struct ProjectDto {
    /// Name
    pub name: String,
    /// Description
    pub description: String,
}

#[derive(InputObject)]
/// The DTO for project
pub struct UpdateProjectDto {
    /// Id
    pub id: Uuid,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
}