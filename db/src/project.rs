use uuid::Uuid;

use model::edit::{Action, Edit};
use model::error::DatabaseError;
use model::files::{File, FileType};
use model::project::Project;

use crate::Database;

impl Database {
    pub async fn create_project(&self, name: String, description: String, user_id: Uuid) -> Result<Project, DatabaseError> {
        let project = Project::create(&self.pool, name, description, user_id).await?;

        let file = File::create(
            &self.pool,
            "main".to_string(),
            "tex".to_string(),
            project.id,
            FileType::Tex,
            None,
        ).await?;

        Edit::create(&self.pool, project.id, Action::Create, user_id, file.id).await?;

        Ok(project)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        Project::get_all(&self.pool).await
    }

    pub async fn get_project_by_uuid(&self, project_id: Uuid) -> Result<Option<Project>, DatabaseError> {
        Project::get_by_uuid(&self.pool, project_id).await
    }
}
