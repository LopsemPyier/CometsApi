use async_graphql::*;
use uuid::Uuid;

#[derive(InputObject)]
/// The DTO for creaating file
pub struct FileDto {
    /// Name
    pub name: String,
    pub project_id: Uuid,
    pub extension: Option<String>,
    pub parent_id: Option<Uuid>,
    pub folder: bool,
}

#[derive(InputObject)]
/// The DTO for updating file
pub struct UpdateFileDto {
    /// Name
    pub name: String,
    pub parent_id: Option<Uuid>,
}