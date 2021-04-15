use async_graphql::*;

#[derive(InputObject)]
/// The DTO for project
pub struct ProjectDto {
    /// Name
    pub name: String,
    /// Description
    pub description: String
}