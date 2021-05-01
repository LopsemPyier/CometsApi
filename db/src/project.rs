use uuid::Uuid;

use model::edit::{Action, Edit};
use model::error::DatabaseError;
use model::file::{File, FileType};
use model::project::Project;
use model::user::User;

use crate::Database;

impl Database {
    pub async fn create_project(&self, name: String, description: String, user_id: Uuid) -> Result<Project, DatabaseError> {
        let project = Project::create(&self.pool, name, description, user_id).await?;

        let file = File::create(
            &self.pool,
            "main".to_string(),
            Some("tex".to_string()),
            project.id,
            FileType::Tex,
            None,
        ).await?;

        Edit::create(&self.pool, project.id, Action::Create, user_id, file.id).await?;

        Ok(project)
    }

    pub async fn add_author(&self, project_id: Uuid, author_id: Uuid) -> Result<bool, DatabaseError> {
        Project::add_author(&self.pool, project_id, author_id).await
    }

    pub async fn remove_author(&self, project_id: Uuid, author_id: Uuid) -> Result<bool, DatabaseError> {
        Project::remove_author(&self.pool, project_id, author_id).await
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        Project::get_all(&self.pool).await
    }

    pub async fn get_projects_for_user(&self, user_id: Uuid) -> Result<Vec<Project>, DatabaseError> {
        User::get_projects_for_user(&self.pool, user_id).await
    }

    pub async fn get_project_by_uuid(&self, project_id: Uuid) -> Result<Option<Project>, DatabaseError> {
        Project::get_by_uuid(&self.pool, project_id).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, DatabaseError> {
        Project::delete(&self.pool, id).await
    }

    pub async fn update(&self, id: Uuid, name: String, description: String) -> Result<Project, DatabaseError> {
        Project::update(&self.pool, id, name, description).await
    }
}
