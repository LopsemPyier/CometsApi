use uuid::Uuid;

use model::error::DatabaseError;
use model::file::{File, FileType};

use crate::Database;

impl Database {
    pub async fn create_file(&self, name: String, extension: String, project_id: Uuid, parent_id: Option<Uuid>, folder: bool) -> Result<File, DatabaseError> {
        let file_type = if folder { FileType::Folder } else { FileType::Tex };

        File::create(&self.pool, name, extension, project_id, file_type, parent_id).await
    }

    pub async fn get_files(&self) -> Result<Vec<File>, DatabaseError> {
        File::get_all(&self.pool).await
    }

    pub async fn get_file_by_uuid(&self, file_id: Uuid) -> Result<Option<File>, DatabaseError> {
        File::get_by_uuid(&self.pool, file_id).await
    }

    pub async fn delete_file(&self, file_id: Uuid) -> Result<bool, DatabaseError> {
        File::delete(&self.pool, file_id).await
    }

    pub async fn update_file(&self, file_id: Uuid, name: String, parent: Option<Uuid>) -> Result<File, DatabaseError> {
        File::update(&self.pool, file_id, name, parent).await
    }
}
