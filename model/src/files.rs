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
}

#[derive(Debug, sqlx::FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub project: Uuid,
    pub file_type: FileType,
    pub extension: String,
    pub parent: Option<Uuid>,
}

impl File {
    pub async fn create(pool: &PgPool, name: String, extension: String, project_id: Uuid, file_type: FileType, parent_id: Option<Uuid>) -> Result<File, DatabaseError> {
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
}