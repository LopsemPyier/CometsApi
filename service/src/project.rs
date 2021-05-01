use async_graphql::{ErrorExtensions, FieldResult};
use uuid::Uuid;

use db::Database;
use schema::error::common::CommonError;
use schema::object::project::ProjectObject;

pub async fn get_all(db: &Database) -> FieldResult<Vec<ProjectObject>> {
    let projects_row = db.get_projects().await?;

    Ok(projects_row.into_iter().map(ProjectObject::from).collect())
}

pub async fn get_for_user(user_id: Uuid, db: &Database) -> FieldResult<Vec<ProjectObject>> {
    let projects_row = db.get_projects_for_user(user_id).await?;

    Ok(projects_row.into_iter().map(ProjectObject::from).collect())
}

pub async fn get_by_uuid(id: Uuid, db: &Database) -> FieldResult<ProjectObject> {
    let project_row = db.get_project_by_uuid(id).await?;

    if let Some(project) = project_row {
        Ok(ProjectObject::from(project))
    } else {
        Err(CommonError::NotFound(id).extend())
    }
}

pub async fn create(db: &Database, user_id: Uuid, name: String, description: String) -> FieldResult<ProjectObject> {
    let project = db.create_project(name, description, user_id).await?;

    Ok(ProjectObject::from(project))
}

pub async fn add_author(db: &Database, project_id: Uuid, author_id: Uuid) -> FieldResult<bool> {
    let res = db.add_author(project_id, author_id).await?;

    Ok(res)
}

pub async fn remove_author(db: &Database, project_id: Uuid, author_id: Uuid) -> FieldResult<bool> {
    let res = db.remove_author(project_id, author_id).await?;

    Ok(res)
}

pub async fn delete(db: &Database, id: Uuid) -> FieldResult<bool> {
    let res = db.delete(id).await?;

    Ok(res)
}

pub async fn update(db: &Database, id: Uuid, name: String, description: String) -> FieldResult<ProjectObject> {
    let project_row = db.update(id, name, description).await?;

    Ok(ProjectObject::from(project_row))
}