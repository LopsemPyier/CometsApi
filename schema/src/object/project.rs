use uuid::Uuid;
use model::project::Project;
use super::user::UserObject;
use async_graphql::Context;
use crate::context::ContextData;
use crate::object::edit::EditObject;
use model::edit::Edit;

#[derive(Debug)]
pub struct ProjectObject {
    pub id: Uuid,
    pub name: String,
    pub description: String
}

#[async_graphql::Object]
/// A project
impl ProjectObject {
    /// The project ID
    pub async fn id(&self) -> Uuid {
        self.id
    }

    /// The project name
    pub async fn name(&self) -> &str {
        &self.name
    }

    /// The project description
    pub async fn description(&self) -> &str {
        &self.name
    }

    /// The authors of the project
    pub async fn authors(&self, ctx: &Context<'_>) -> Vec<UserObject> {
        let data = ctx.data::<ContextData>().unwrap();
        let authors = Project::get_authors(&data.db.pool, self.id).await.unwrap();
        authors.into_iter().map(UserObject::from).collect()
    }

    /// The edits of the project
    pub async fn edits(&self, ctx: &Context<'_>) -> Vec<EditObject> {
        let data = ctx.data::<ContextData>().unwrap();
        let edits = Edit::get_for_project(&data.db.pool, self.id).await.unwrap();
        edits.into_iter().map(EditObject::from).collect()
    }

    /// The last edit of the project
    pub async fn last_edit(&self, ctx: &Context<'_>) -> EditObject {
        let data = ctx.data::<ContextData>().unwrap();
        let edit = Edit::get_last_for_project(&data.db.pool, self.id).await.unwrap();
        EditObject::from(edit)
    }
}

impl From<Project> for ProjectObject {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            description: project.description
        }
    }
}

/*impl From<Vec<Project>> for Vec<ProjectObject> {
    fn from(projects: Vec<Project>) -> Self {
        projects.into_iter().map(|project| Self::from(project)).collect()
    }
}*/