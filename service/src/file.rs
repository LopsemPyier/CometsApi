use async_graphql::{ErrorExtensions, FieldResult};
use uuid::Uuid;

use db::Database;
use schema::error::common::CommonError;
use schema::object::file::FileObject;

pub async fn get_all(db: &Database) -> FieldResult<Vec<FileObject>> {
    let files_row = db.get_files().await?;

    Ok(files_row.into_iter().map(FileObject::from).collect())
}

pub async fn get_by_uuid(id: Uuid, db: &Database) -> FieldResult<FileObject> {
    let file_row = db.get_file_by_uuid(id).await?;

    if let Some(file) = file_row {
        Ok(FileObject::from(file))
    } else {
        Err(CommonError::NotFound(id).extend())
    }
}

pub async fn create(db: &Database, name: String, extension: String, project_id: Uuid, parent_id: Option<Uuid>) -> FieldResult<FileObject> {
    let project = db.create_file(name, extension, project_id, parent_id).await?;

    Ok(FileObject::from(project))
}