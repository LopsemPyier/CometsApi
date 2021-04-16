use uuid::Uuid;
use sqlx::PgPool;
use crate::error::DatabaseError;
use crate::user::User;

#[derive(Debug, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String
}

impl Project {
    pub async fn create(pool: &PgPool, name: String, description: String, user_id: Uuid) -> Result<Project, DatabaseError> {
        let result = sqlx::query_file_as!(
			Project,
			"src/sql/project/create.sql",
            name,
            description
		)
            .fetch_one(pool)
            .await;

        match result {
            Ok(project) => {
                Project::add_author(pool, project.id, user_id).await?;
                Ok(project)
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(err.into())
            }
        }
    }

    pub async fn add_author(pool: &PgPool, project_id: Uuid, user_id: Uuid) -> Result<(), DatabaseError> {
        let result = sqlx::query_file!(
            "src/sql/project/add_author.sql",
            user_id,
            project_id
        )
            .execute(pool)
            .await;

        match result {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(err.into())
            }
        }
    }

    pub async fn get_authors(pool: &PgPool, project_id: Uuid) -> Result<Vec<User>, DatabaseError> {
        let authors_row = sqlx::query_file_as!(
			User,
			"src/sql/project/get_authors.sql",
			project_id
		)
            .fetch_all(pool)
            .await?;

        Ok(authors_row)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Project>, DatabaseError> {
        let projects_row = sqlx::query_file_as!(
			Project,
			"src/sql/project/get_all.sql"
		)
            .fetch_all(pool)
            .await?;

        Ok(projects_row)
    }

    pub async fn get_by_uuid(pool: &PgPool, id: Uuid) -> Result<Option<Project>, DatabaseError> {
        let project_row = sqlx::query_file_as!(
			Project,
			"src/sql/project/get_by_uuid.sql",
			id
		)
            .fetch_optional(pool)
            .await?;

        Ok(project_row)
    }
}