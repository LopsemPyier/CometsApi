use crate::Database;
use model::project::Project;
use model::error::DatabaseError;
use uuid::Uuid;
use model::edit::{Edit, Action};

impl Database {
    pub async fn create_project(&self, name: String, description: String, user_id: Uuid) -> Result<Project, DatabaseError> {
        let project = Project::create(&self.pool, name, description, user_id).await?;

        Edit::create(&self.pool, project.id, Action::Create, user_id).await?;

        Ok(project)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        Project::get_all(&self.pool).await
    }

    pub async fn get_project_by_uuid(&self, project_id: Uuid) -> Result<Option<Project>, DatabaseError> {
        Project::get_by_uuid(&self.pool, project_id).await
    }
}
