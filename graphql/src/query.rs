use super::context::ContextData;

use model::user::User;

use service::user;

use async_graphql::{ Context, FieldResult, ID };
use uuid::Uuid;

pub struct Query;

#[async_graphql::Object]
impl Query{
	pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
		let data = ctx.data::<ContextData>()?;
		user::get_all(&data.db).await
	}

	pub async fn user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<User> {
		let data = ctx.data::<ContextData>()?;
        let user_id = Uuid::parse_str(id.as_str()).unwrap();
		user::get_by_uuid(user_id, &data.db).await
	}
}