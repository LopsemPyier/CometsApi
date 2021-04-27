use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DatabaseError;

#[derive(Debug, sqlx::Type)]
#[sqlx(rename = "file_type")]
pub enum FileType {
    #[sqlx(rename = "IMAGE")]
    Image,
    #[sqlx(rename = "TEX")]
    Tex,
    #[sqlx(rename = "PDF")]
    Pdf,
    #[sqlx(rename = "FOLDER")]
    Folder,
}

#[derive(Debug, sqlx::FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub project: Uuid,
    pub file_type: FileType,
    pub extension: Option<String>,
    pub parent: Option<Uuid>,
}

impl File {
    pub async fn create(pool: &PgPool, name: String, extension: Option<String>, project_id: Uuid, file_type: FileType, parent_id: Option<Uuid>) -> Result<File, DatabaseError> {
        let result = sqlx::query_file_as!(
			File,
			"src/sql/file/create.sql",
            name,
            project_id,
            extension,
            file_type as FileType,
            parent_id
		)
            .fetch_one(pool)
            .await;

        match result {
            Ok(file) => {
                Ok(file)
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(err.into())
            }
        }
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<File>, DatabaseError> {
        let files_row = sqlx::query_file_as!(
			File,
			"src/sql/file/getall.sql"
		)
            .fetch_all(pool)
            .await?;

        Ok(files_row)
    }

    pub async fn get_by_uuid(pool: &PgPool, id: Uuid) -> Result<Option<File>, DatabaseError> {
        let file_row = sqlx::query_file_as!(
			File,
			"src/sql/file/get.sql",
			id
		)
            .fetch_optional(pool)
            .await?;

        Ok(file_row)
    }

    pub async fn get_for_project(pool: &PgPool, project_id: Uuid) -> Result<Vec<File>, DatabaseError> {
        let files_row = sqlx::query_file_as!(
			File,
			"src/sql/file/getForProject.sql",
			project_id
		)
            .fetch_all(pool)
            .await?;

        Ok(files_row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, DatabaseError> {
        sqlx::query_file!(
            "src/sql/file/delete.sql",
            id
        )
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn update(pool: &PgPool, id: Uuid, name: String, parent: Option<Uuid>) -> Result<File, DatabaseError> {
        let file_row = sqlx::query_file_as!(
            File,
            "src/sql/file/update.sql",
            id,
            name,
            parent
        )
            .fetch_one(pool)
            .await?;

        Ok(file_row)
    }
}