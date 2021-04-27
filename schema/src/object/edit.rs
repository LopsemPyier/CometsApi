use async_graphql::{Context, ErrorExtensions, FieldResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use model::edit::{Action, Edit};
use model::file::File;
use model::user::User;

use crate::context::ContextData;
use crate::error::common::CommonError;
use crate::object::file::FileObject;
use crate::object::user::UserObject;

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum ActionEnum {
    CREATE,
    DELETE,
    EDIT,
}

#[derive(Debug)]
pub struct EditObject {
    pub id: Uuid,
    pub action_type: ActionEnum,
    pub create_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub file_id: Uuid,
}

#[async_graphql::Object]
impl EditObject {
    pub async fn id(&self) -> Uuid {
        self.id
    }

    pub async fn action_type(&self) -> &ActionEnum {
        &self.action_type
    }

    pub async fn create_at(&self) -> DateTime<Utc> {
        self.create_at
    }

    pub async fn author(&self, ctx: &Context<'_>) -> Option<UserObject> {
        let data = ctx.data::<ContextData>().unwrap();
        let user = User::get_by_uuid(&data.db.pool, self.user_id).await.unwrap();
        if let Some(user) = user {
            Some(UserObject::from(user))
        } else {
            None
        }
    }

    pub async fn file(&self, ctx: &Context<'_>) -> FieldResult<FileObject> {
        let data = ctx.data::<ContextData>().unwrap();
        let file = File::get_by_uuid(&data.db.pool, self.file_id).await.unwrap();
        if let Some(file) = file {
            Ok(FileObject::from(file))
        } else {
            Err(CommonError::NotFound(self.file_id).extend())
        }
    }
}

impl From<Action> for ActionEnum {
    fn from(action: Action) -> Self {
        match action {
            Action::Create => Self::CREATE,
            Action::Delete => Self::DELETE,
            Action::Edit => Self::EDIT
        }
    }
}

impl From<Edit> for EditObject {
    fn from(edit: Edit) -> Self {
        Self {
            id: edit.id,
            action_type: ActionEnum::from(edit.action_type),
            create_at: edit.create_at,
            user_id: edit.author_id,
            file_id: edit.file,
        }
    }
}