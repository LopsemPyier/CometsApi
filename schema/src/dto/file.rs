use async_graphql::*;
use uuid::Uuid;

#[derive(InputObject)]
/// The DTO for project
pub struct FileDto {
    /// Name
    pub name: String,
    pub project_id: Uuid,
    pub extension: String,
    pub parent_id: Option<Uuid>,
}