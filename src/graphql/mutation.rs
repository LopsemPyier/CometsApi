use super::context::ContextData;

use crate::model;

use model::user::User;

use async_graphql::{ Context, FieldResult, ID };
use uuid::Uuid;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation{
	pub async fn create_user(&self, ctx: &Context<'_>, username: String, password: String, email: String) -> FieldResult<User> {
		let data = ctx.data::<ContextData>()?;
		let user = data.db.create_user(username, password, email).await?;

		Ok(user.into())
	}
}