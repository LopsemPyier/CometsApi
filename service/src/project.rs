use db::Database;
use async_graphql::{FieldResult, Error};
use uuid::Uuid;
use schema::object::project::ProjectObject;

pub async fn get_all(db: &Database) -> FieldResult<Vec<ProjectObject>> {
    let projects_row = db.get_projects().await?;

    Ok(projects_row.into_iter().map(ProjectObject::from).collect())
}

pub async fn get_by_uuid(id: Uuid, db: &Database) -> FieldResult<ProjectObject> {
    let project_row = db.get_project_by_uuid(id).await?;

    if let Some(project) = project_row {
        Ok(ProjectObject::from(project))
    } else {
        Err(Error::new("Project not found"))
    }
}

pub async fn create(db: &Database, user_id: Uuid, name: String, description: String) -> FieldResult<ProjectObject> {
    let project = db.create_project(name, description, user_id).await?;



    Ok(ProjectObject::from(project))
}