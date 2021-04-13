use super::context::ContextData;

use crate::model;

use model::user::User;

use async_graphql::{ Context, FieldResult, ID };
use uuid::Uuid;

pub struct Query;

#[async_graphql::Object]
impl Query{
	pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
		let data = ctx.data::<ContextData>()?;
		let users = data.db.get_users().await?;

		Ok(users.into())
	}

	pub async fn user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<User> {
		let data = ctx.data::<ContextData>()?;
        let user_id = Uuid::parse_str(id.as_str()).unwrap();
		let user = data.db.get_user_by_uuid(user_id).await?;

		Ok(user.into())
	}
}