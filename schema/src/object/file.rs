use async_graphql::{Context, ErrorExtensions, FieldResult};
use uuid::Uuid;

use model::files::{File, FileType};
use model::project::Project;

use crate::context::ContextData;
use crate::error::common::CommonError;
use crate::object::project::ProjectObject;

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum FileTypeEnum {
    IMAGE,
    TEX,
    PDF,
}

#[derive(Debug)]
pub struct FileObject {
    pub id: Uuid,
    pub file_type: FileTypeEnum,
    pub name: String,
    pub project_id: Uuid,
    pub extension: String,
    pub parent_id: Option<Uuid>,
}


#[async_graphql::Object]
impl FileObject {
    pub async fn id(&self) -> Uuid {
        self.id
    }

    pub async fn file_type(&self) -> &FileTypeEnum {
        &self.file_type
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn extension(&self) -> &str {
        &self.extension
    }

    pub async fn project(&self, ctx: &Context<'_>) -> FieldResult<ProjectObject> {
        let data = ctx.data::<ContextData>().unwrap();
        let project = Project::get_by_uuid(&data.db.pool, self.project_id).await.unwrap();
        if let Some(project) = project {
            Ok(ProjectObject::from(project))
        } else {
            Err(CommonError::NotFound(self.project_id).extend())
        }
    }

    pub async fn parent(&self, ctx: &Context<'_>) -> FieldResult<Option<FileObject>> {
        if let Some(parent_id) = self.parent_id {
            let data = ctx.data::<ContextData>().unwrap();
            let parent = File::get_by_uuid(&data.db.pool, parent_id).await.unwrap();
            if let Some(parent) = parent {
                Ok(Some(FileObject::from(parent)))
            } else {
                Err(CommonError::NotFound(parent_id).extend())
            }
        } else {
            Ok(None)
        }
    }
}

impl From<FileType> for FileTypeEnum {
    fn from(file_type: FileType) -> Self {
        match file_type {
            FileType::Image => Self::IMAGE,
            FileType::Tex => Self::TEX,
            FileType::Pdf => Self::PDF
        }
    }
}

impl From<File> for FileObject {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            file_type: FileTypeEnum::from(file.file_type),
            name: file.name,
            project_id: file.project,
            extension: file.extension,
            parent_id: file.parent,
        }
    }
}